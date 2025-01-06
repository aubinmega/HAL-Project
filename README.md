# HAL-Project

The project utilizes Rust to create a Hardware Abstraction Layer (HAL) for controlling microcontrollers like the Atmega328p (Arduino) and ESP8266.

## Functionality

Configures the Arduino board LED (connected to pin D13) as the output and makes it blink on and off every second.
Provides SPI and USART communication interfaces for Atmega328p.
Implements GPIO and SPI stubs for ESP8266 for future extensions.
Requirements

## Requirements

- cargo
- rust
- nightly toolchain
- simAVR as the emulator
- GDB for degguging
- AVR-GCC for Atmega328p builds
- Xtensa GCC toolchain for ESP8266 build

### PS : Cargo Building command

cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
cargo +nightly build -Z build-std=core,panic_abort --target avr-unknown-gnu-atmega328 --release

cargo +nightly build -Z build-std=core --target ./xtensa-esp8266-none-elf.json --release

### Highlights

Atmega328p:
    Internal clock setup for full-speed operation.
    GPIO control for LED toggling.
    SPI and USART communication.

ESP8266:
    GPIO stubs for controlling pins.
    SPI stubs for communication.

### Authors

- [Aubin NCUTI](https://github.com/aubinmega)
- [Charles NGUYEN](https://github.com/Sylkka)
- [Anton NOMED](https://github.com/Coraz0nn)
