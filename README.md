# `esp32-c3-rs`

[ESP32-C3](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c3/esp32-c3-devkitm-1/user_guide.html)
is a low-power and highly-integrated MCU-based solution that supports 2.4 GHz Wi-Fi and
Bluetooth® Low Energy (Bluetooth LE). It has a 32-bit RISC-V single-core processor, up to 160 MHz, 384 KB ROM, 400 KB SRAM (16 KB for cache) and 8 KB SRAM in RTC. The ISA is `riscv32imc-unknown-none-elf` (no support for atomics).

![esp32-c3-dual-dk](https://github.com/gdamjan/esp32-c3-devkit/assets/81654/0d84a0fa-a53b-472a-b6e8-a2137764738c)

## Preparation

- [`rustup`](https://rustup.rs/) - is recommended to install rust and its components
- `cargo install cargo-binutils` - for `cargo size` and `cargo objdump -- --disassemble`, etc…
- `cargo install cargo-espflash` - flash and monitor using the [`espflash project`](https://github.com/esp-rs/espflash/tree/main/cargo-espflash)

## Flash and monitor

- `cargo build --release`
- `cargo espflash flash --release`
- `cargo espflash monitor`

## VS Code settings

The repo will also suggest common extensions for VS Code:
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [crates helper](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

and some settings to instruct rust-analyzer to only run for the `riscv32imc-unknown-none-elf` target.

## References

- [ESP32-C3 Series Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c3_datasheet_en.pdf)
- https://github.com/esp-rs
- https://docs.esp-rs.org/book/
