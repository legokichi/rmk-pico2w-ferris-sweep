# Repository Guidelines

## Project Structure & Module Organization
- `src/central.rs` and `src/peripheral.rs` are the two no_std firmware entry points (split keyboard).
- `build.rs` generates Vial config at build time and wires linker scripts; `memory.x` holds the RP2350 memory map.
- `keyboard.toml` + `vial.json` are the source-of-truth for layout and Vial metadata.
- `cyw43-firmware/` contains Wi-Fi/BLE firmware blobs downloaded by the build script.
- `target/` is build output; keep it untracked.

## Build, Test, and Development Commands
- `cargo make build` builds the release firmware and installs required tools (`llvm-tools`, `flip-link`).
- `cargo make uf2` produces both `pico2wh-central.uf2` and `pico2wh-peripheral.uf2` (also available as `cargo make uf2-central` / `cargo make uf2-peripheral`).
- `cargo build --release --bin central` or `--bin peripheral` is the minimal build path without cargo-make.
- Toolchain is defined in `rust-toolchain.toml` (stable + `thumbv6m-none-eabi`).

## Coding Style & Naming Conventions
- Rust 2021 edition; default `rustfmt` style (4-space indent, trailing commas).
- Follow Rust naming: `snake_case` modules/functions, `CamelCase` types, `SCREAMING_SNAKE_CASE` consts.
- Keep embedded constraints in mind (`#![no_std]`); avoid heap-heavy patterns unless required.

## Testing Guidelines
- No automated tests are currently present.
- If adding tests, place `#[cfg(test)]` modules alongside the code and use clear names like `test_ble_pairing_smoke`.

## Commit & Pull Request Guidelines
- No commit history exists yet; use concise, imperative messages (e.g., "Add split BLE config").
- PRs should describe hardware/board assumptions and note any changes to `keyboard.toml` or `vial.json`.
- Avoid committing generated artifacts (`pico2wh-*.hex`, `pico2wh-*.uf2`) unless explicitly requested.

## Configuration & Firmware Assets
- `build.rs` downloads `cyw43-firmware` assets if missing; for offline builds use `--features skip-cyw43-firmware` or pre-populate `cyw43-firmware/`.
