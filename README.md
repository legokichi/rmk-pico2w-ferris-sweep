# rmk-pico2w-ferris-sweep

Split keyboard firmware for RP2350 (Pico 2 W) using Rust (`no_std`).

## Requirements

- Rust toolchain per `rust-toolchain.toml` (stable + target `thumbv8m.main-none-eabihf`)
- `rustup` installed
- Optional: `cargo-make` for the convenience tasks in `Makefile.toml`

## Setup

1. Install Rust via rustup:
   - https://rustup.rs
2. Install the toolchain components and target:
   ```sh
   rustup toolchain install stable
   rustup target add thumbv8m.main-none-eabihf
   rustup component add rust-src rustfmt llvm-tools
   ```
3. Optional: install `cargo-make` for `cargo make` tasks:
   ```sh
   cargo install flip-link cargo-make
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
   ```

## Build

Minimal build without cargo-make:

```sh
cargo build --release --bin central
cargo build --release --bin peripheral
```

With cargo-make:

```sh
cargo make build
```

Generate UF2 files:

```sh
cargo make uf2
```

Outputs:
- `pico2wh-central.uf2`
- `pico2wh-peripheral.uf2`

## Firmware assets (CYW43)

`build.rs` downloads CYW43 firmware blobs into `cyw43-firmware/` if missing.
For offline builds, either pre-populate `cyw43-firmware/` or use:

```sh
cargo build --release --features skip-cyw43-firmware --bin central
cargo build --release --features skip-cyw43-firmware --bin peripheral
```

## Flash UF2 to Pico 2 W

1. Hold the **BOOTSEL** button on the board.
2. While holding it, plug the USB cable into your computer.
3. A USB mass storage drive (e.g. `RPI-RP2`) will appear.
4. Copy the UF2 file to that drive.
5. The board will reboot automatically and the flash is complete.

Split keyboard note: flash `pico2wh-central.uf2` to the central half and
`pico2wh-peripheral.uf2` to the peripheral half.

If the drive does not show up, try a different USB cable/port and re-check
the BOOTSEL timing.

## BLE debug tips

- Device name length is limited to 22 bytes. Longer names cause a panic at boot.
- Run with RTT logs:
  ```sh
  DEFMT_LOG=debug cargo run --release --bin central
  ```
  (This uses `probe-rs run` from `.cargo/config.toml`. For `cargo-embed`, see `Embed.toml`.)
- Check advertising from the host:
  ```sh
  bluetoothctl scan on
  bluetoothctl devices | rg -i rmk
  ```
- Pair from the host:
  ```sh
  bluetoothctl
  pair XX:XX:XX:XX:XX:XX
  trust XX:XX:XX:XX:XX:XX
  connect XX:XX:XX:XX:XX:XX
  ```
- Capture host-side pairing logs:
  ```sh
  sudo btmon -w host-btmon.log
  ```
  Then retry pairing and inspect the log for:
  `LE Enhanced Connection Complete`, `SMP: Pairing`, `Encryption Change`.
- Clear host-side pairing info (Ubuntu / BlueZ):
  - Remove a single device:
    ```sh
    bluetoothctl devices | grep rmk
    bluetoothctl remove XX:XX:XX:XX:XX:XX
    ```
  - If it keeps coming back, remove BlueZ cache directly (this deletes stored keys):
    ```sh
    bluetoothctl list
    sudo systemctl stop bluetooth
    sudo rm -rf /var/lib/bluetooth/<adapter>/<device>
    sudo systemctl start bluetooth
    ```
- If bonding info is stale, set `clear_storage = true` once, boot, then revert to `false`.

### Cleanup debug logs

If you added BLE debug logs during investigation, you can remove them with:

```sh
./scripts/cleanup-debug.sh
```

This applies:
- `scripts/cleanup-debug-root.patch` (removes `embedded-io` dependency)
- `scripts/cleanup-debug-rmk.patch` (removes debug logs in the rmk submodule)

## Split BLE topology (central/peripheral/host)

The central half acts as a hub:

```
[Peripheral (left/right)]  <-- BLE (split link) -->  [Central]  <-- BLE HID -->  [Host PC]
                                                     |
                                                     +-- USB HID (when USB is connected)
```

- Central connects to the other half as BLE central (split link).
- Central advertises to the host PC as a BLE HID peripheral.
- Peripheral never connects to the host directly; it sends key data to central.

Implementation note: in BLE split mode, the peripheral must send key events even
when `CONNECTION_STATE` (host connection) is false. The split link is between
peripheral ↔ central, and the central may not be connected to the host yet.
Gating peripheral sends on host connection drops key events and makes presses
look like releases. The BLE split path therefore bypasses the host-connection
gate for key events.

## RP2350 SPI note (Pico 2 W)

If you see split BLE packets arriving as all-zeros (pressed → false), slow down
the CYW43 PIO SPI clock. This repo applies `RM2_CLOCK_DIVIDER * 2` in
`rmk-macro/src/chip_init.rs` to keep the GSPI clock at or below 25MHz, which
fixes bit flips observed on RP2350.

## Project layout

- `src/central.rs` / `src/peripheral.rs`: firmware entry points
- `keyboard.toml` + `vial.json`: layout + Vial metadata source of truth
- `build.rs`: config generation + linker script wiring
- `memory.x`: RP2350 memory map
