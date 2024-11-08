// main.rs
#![no_std] 
#![no_main] 

mod delay;
mod gpio;

use avr_device::entry; // Import de l'attribut #[entry] pour définir le point d'entrée de l'application
use gpio::gpio::{configure_pin, write_pin, Mode}; // Import des fonctions du module gpio
use delay::delay::delay_ms; // Import de la fonction de délai du module delay

#[entry]
fn main() -> ! {
    // Configuration de la broche 5 en tant que sortie
    configure_pin(5, Mode::Output);

    // Boucle infinie pour allumer et éteindre la LED avec un délai
    loop {
        write_pin(5, true); // Allume la LED
        delay_ms(1000); // Pause de 1000 millisecondes
        write_pin(5, false); // Éteint la LED
        delay_ms(1000); // Pause de 1000 millisecondes
    }
}





