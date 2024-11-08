// gpio.rs
pub mod gpio {
    use core::ptr; // Utilisé pour lire et écrire directement en mémoire (opérations non sécurisées)

    const DDRB: *mut u8 = 0x24 as *mut u8; // Registre pour configurer les directions des pins (entrée/sortie)
    const PORTB: *mut u8 = 0x25 as *mut u8; // Registre pour écrire des valeurs logiques sur les pins

    // Enumération pour définir les modes de broche
    pub enum Mode {
        Input,
        Output,
    }

    // Fonction pour configurer une broche comme entrée ou sortie
    pub fn configure_pin(pin: u8, mode: Mode) {
        unsafe {
            // Lecture du registre DDRB actuel
            let current_ddrb = ptr::read_volatile(DDRB);
            // Selon le mode, on configure la broche en entrée ou en sortie
            match mode {
                Mode::Input => ptr::write_volatile(DDRB, current_ddrb & !(1 << pin)), // Configuration en entrée
                Mode::Output => ptr::write_volatile(DDRB, current_ddrb | (1 << pin)), // Configuration en sortie
            }
        }
    }

    // Fonction pour écrire une valeur (haute ou basse) sur une broche
    pub fn write_pin(pin: u8, value: bool) {
        unsafe {
            // Lecture du registre PORTB actuel
            let current_portb = ptr::read_volatile(PORTB);
            // Selon la valeur, on met la broche à l'état haut (true) ou bas (false)
            if value {
                ptr::write_volatile(PORTB, current_portb | (1 << pin)); // État haut
            } else {
                ptr::write_volatile(PORTB, current_portb & !(1 << pin)); // État bas
            }
        }
    }
}
