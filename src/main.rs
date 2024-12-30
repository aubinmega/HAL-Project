#![no_std]
#![no_main]

#[cfg(feature = "atmega328p")]
mod atmega328p;

#[cfg(feature = "esp8266")]
mod esp8266;

mod spi;
mod usart;

#[cfg(feature = "atmega328p")]
use avr_device::entry;

#[cfg(feature = "atmega328p")]
use crate::atmega328p::{setup_internal_clock, setup_gpio, led_on, led_off, delay_ms};

#[cfg(feature = "esp8266")]
use crate::esp8266::{setup_gpio, set_gpio_high, set_gpio_low};

#[cfg(feature = "atmega328p")]
use crate::spi::atmega328p_spi::{init_spi, transfer};

#[cfg(feature = "esp8266")]
use crate::spi::esp8266_spi::{init_spi, transfer};

#[entry]
fn main() -> ! {
    #[cfg(feature = "atmega328p")]
    {
        setup_internal_clock();
        setup_gpio();
        init_spi();
    }

    #[cfg(feature = "esp8266")]
    {
        setup_gpio(2);
        init_spi();
    }

    loop {
        #[cfg(feature = "atmega328p")]
        {
            led_on();
            delay_ms(1000);
            led_off();
            delay_ms(1000);
        }

        #[cfg(feature = "esp8266")]
        {
            set_gpio_high(2);
            delay_ms(1000);
            set_gpio_low(2);
            delay_ms(1000);
        }
    }
}
