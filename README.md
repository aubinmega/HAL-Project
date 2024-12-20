# HAL-Project : Making the Arduino LED Blink using embedded Rust and simAVR

The project utilizes the 'arduino-hal' crate to control the LED of an Arduino by making it blink on and off every second.

## Functionality

The program configures the Arduino board LED (connected to pin D13 on most Arduino boards) as the output.

## Requirements

- cargo
- rust
- nightly toolchain
- simAVR as the emulator
- GDB for degguging

### PS : Cargo Building command

cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
cargo +nightly build -Z build-std=core,panic_abort --target avr-unknown-gnu-atmega328 --release

### Authors

- [Aubin NCUTI](https://github.com/aubinmega)
- [Charles NGUYEN](https://github.com/Sylkka)
- [Anton NOMED](https://github.com/Coraz0nn)


[CORRECTION USART] (Don't hesitate to remove this part)
You could try implementing the internal clock of the Atmega as well
One target is missing !

[CORRECTION SPI] (don't hesitate to remove this part)
You didn't implement the SPI feature.
Your second target is missing.