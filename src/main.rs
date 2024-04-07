use clap::{App, Arg};
use std::{fs, io};

fn caesar_cipher(input: &str, key: i32) -> String {
    input.to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let first_letter = b'A';
            let shifted = ((c as u8 - first_letter + (key.rem_euclid(26) as u8))).rem_euclid(26) + first_letter;
            shifted as char
        })
        .collect()
}

fn main() -> io::Result<()> {
    let matches = App::new("Cezar")
        .version("1.0")
        .about("Lab2 KK")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .help("Sets the input plaintext file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .help("Sets the output encrypted file")
                .takes_value(true)
        )
        .arg(
            Arg::new("encrypt")
                .short('e')
                .long("encrypt")
                .help("Encrypts the input file"),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .help("Decrypts the input file"),
        )
        .arg(
            Arg::with_name("key")
                .short('k')
                .long("key")
                .value_name("FILE")
                .help("Sets the encryption key (dictionary) file")
                .takes_value(true)
        ).get_matches();

    let is_encrypt = matches.is_present("encrypt");
    let is_decrypt = matches.is_present("decrypt");
    let key = matches.value_of("key").unwrap().parse::<i32>().expect("Key must be an integer");
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    let input_text = fs::read_to_string(input_file).expect("Error reading input file");
    let processed_text = if is_encrypt {
        caesar_cipher(&input_text, key)
    } else if is_decrypt {
        caesar_cipher(&input_text, -key)
    } else {
        panic!("Either encrypt or decrypt must be specified");
    };

    fs::write(output_file, processed_text).expect("Error writing to output file");

    Ok(())
}
