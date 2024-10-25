#![no_std] // disable the standard library
#![no_main] // Not relevant because this usually instructs the compiler to not look for a 'main' function as the entry point, but in our case, we still used the 'main' function

use panic_halt as _;  // panic handler. In this case , it will just halt(stop) the program

#[arduino_hal::entry] //  This is the entry point for the Arduino HAL
fn main() -> ! {   
    let dp = arduino_hal::Peripherals::take().unwrap();  // Take the peripherals from the Arduino HAL
    let pins = arduino_hal::pins!(dp); //  Create a pins object from the peripherals

    let mut led = pins.d13.into_output();  //  Create a pin object for the LED pin and set it to output

    loop { // //   This is an infinite loop that will blink the LED
        led.set_high(); // Turn on the led by setting the pin D13 to a high voltage         
        arduino_hal::delay_ms(1000);  // wait for 1ooo ms, which is 1 second
        led.set_low();              //setting the pin to a low voltage, Turn off the led
        arduino_hal::delay_ms(1000); // wait for another second
    } 
}
