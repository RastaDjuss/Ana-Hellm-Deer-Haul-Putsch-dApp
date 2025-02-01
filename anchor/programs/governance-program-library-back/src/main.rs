use libc::c_void;
use std::ptr; // Pour `ptr::null_mut`

// Définir les constantes manuellement
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x02;
const MAP_ANONYMOUS: i32 = 0x20;
const MAP_FAILED: *mut c_void = -1isize as *mut c_void;

// Importation explicite des fonctions C
extern "C" {
    pub fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: i32,
        flags: i32,
        fd: i32,
        offset: i64,
    ) -> *mut c_void;

    pub fn munmap(addr: *mut c_void, len: usize) -> i32;
}

fn main() {
    let size: usize = 4096; // Taille de la mémoire à allouer (4 Ko)

    unsafe {
        // Appel système mmap
        let ptr = mmap(
            ptr::null_mut(),            // Aucune adresse de base demandée
            size,                       // Taille à allouer
            PROT_READ | PROT_WRITE,     // Protection : lecture et écriture
            MAP_PRIVATE | MAP_ANONYMOUS, // Flags : mémoire privée et anonyme
            -1,                         // Pas de fichier
            0,                          // Pas de déplacement
        );

        // Vérification si mmap a échoué
        if ptr == MAP_FAILED {
            eprintln!("Erreur : mmap a échoué");
            std::process::exit(1);
        } else {
            println!("Mémoire allouée à l'adresse : {:?}", ptr);
        }

        // Appel système munmap pour libérer la mémoire
        if munmap(ptr, size) != 0 {
            eprintln!("Erreur : munmap a échoué");
            std::process::exit(1);
        }

        println!("Mémoire désallouée avec succès!");
    } // Fin du bloc `unsafe`
}