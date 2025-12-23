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

## Project layout

- `src/central.rs` / `src/peripheral.rs`: firmware entry points
- `keyboard.toml` + `vial.json`: layout + Vial metadata source of truth
- `build.rs`: config generation + linker script wiring
- `memory.x`: RP2350 memory map
