mod caesar;
mod brute_force;
mod affine;

use clap::{App, Arg};
use std::{fs, io};
use crate::affine::{decrypt, encrypt, has_modular_inverse};
use crate::brute_force::{brute_force_affine, brute_force_caesar};
use crate::caesar::cipher;


fn main() -> io::Result<()> {
    let matches = App::new("Cezar")
        .version("1.0")
        .about("Lab2 KK")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .help("Sets the input plaintext file")
            .takes_value(true)
        )
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .help("Sets the output encrypted file")
            .takes_value(true)
        )
        .arg(Arg::new("encrypt")
                 .short('e')
                 .long("encrypt")
                 .help("Encrypts the input file"),
        )
        .arg(Arg::new("decrypt")
                 .short('d')
                 .long("decrypt")
                 .help("Decrypts the input file"),
        )
        .arg(Arg::with_name("key")
            .short('k')
            .long("key")
            .help("Sets the encryption key (dictionary) file")
            .takes_value(true)
        )
        .arg(Arg::with_name("caesar")
            .long("caesar")
            .help("Sets cipher mode to caesar")
        )
        .arg(Arg::with_name("affine")
            .long("affine")
            .help("Sets cipher mode to affine")
        )
        .arg(Arg::with_name("brute_force")
            .long("brute")
            .help("Runs bruteforce attack on input")
        )
        .arg(Arg::with_name("a")
            .short('a')
            .help("Sets variable a for affine cipher")
            .takes_value(true)
        )
        .arg(Arg::with_name("b")
            .short('b')
            .help("Sets variable b for affine cipher")
            .takes_value(true)
        )
        .get_matches();

    let input_file = matches.value_of("input").expect("Input file is required");
    let input_text = fs::read_to_string(input_file).expect("Error reading input file");


    let is_encrypt = matches.is_present("encrypt");
    let is_decrypt = matches.is_present("decrypt");
    let is_caesar = matches.is_present("caesar");
    let is_affine = matches.is_present("affine");
    let is_brute_force = matches.is_present("brute_force");


    let (a, b) = if is_affine && ! is_brute_force {
        let a = matches.value_of("a")
            .expect("Variable a is requiered for affine cypher")
            .parse::<i32>()
            .expect("Variable must be an integer");
        if !has_modular_inverse(a) {
            panic!("Variable a must be coprime with 26 to be used in the affine cipher");
        }
        let b = matches.value_of("b")
            .expect("Variable b is required for affine cypher")
            .parse::<i32>()
            .expect("variable must be an integer");
        (a, b)
    } else {
        (0, 0)
    };

    let key = if matches.is_present("key") {
        matches.value_of("key")
            .expect("Key is necessary")
            .parse::<i32>()
            .expect("Key must be an integer")
    } else {
        0
    };


    let processed_text = if is_brute_force {
        if is_caesar {
            format!("{:?}", brute_force_caesar(&input_text))
        } else if is_affine {
            format!("{:?}", brute_force_affine(&input_text))
        } else {
            panic!("No brute force method specified!")
        }
    } else if is_encrypt || is_decrypt {
        let encrypt_or_decrypt = if is_encrypt { "Encrypting" } else { "Decrypting" };
        println!("{} with {} cipher...", encrypt_or_decrypt, if is_caesar { "Caesar" } else { "Affine" });

        match (is_caesar, is_affine) {
            (true, false) => cipher(&input_text, if is_encrypt { key } else { -key }),
            (false, true) => if is_encrypt {
                encrypt(&input_text, a, b)
            } else {
                decrypt(&input_text, a, b).unwrap_or_else(|| panic!("Decryption failed, invalid input"))
            }
            _ => panic!("Either caesar or affine must be specified")
        }
    } else {
        panic!("Encryption or decryption must be specified!")
    };

    if let Some(output_file) = matches.value_of("output") {
        fs::write(output_file, &processed_text)
            .expect("Failed to write to output file");
    }

    Ok(())
}
