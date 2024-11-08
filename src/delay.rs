// delay.rs
pub mod delay {
    use core::ptr; // Import pour accéder aux opérations de lecture mémoire directe

    // Fonction pour créer un délai en millisecondes
    pub fn delay_ms(ms: u16) {
        // Boucle externe pour le nombre de millisecondes désiré
        for _ in 0..ms {
            // Boucle interne pour ajuster la durée, en fonction de la fréquence du microcontrôleur
            for _ in 0..16_000 {
                // Lecture d'une adresse mémoire arbitraire pour créer une pause (busy-waiting)
                unsafe { ptr::read_volatile(0x00 as *const u8); }
            }
        }
    }
}