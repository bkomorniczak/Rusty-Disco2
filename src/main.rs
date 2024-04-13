mod caesar;
mod brute_force;
mod affine;

use clap::{App, Arg};
use std::{fs, io};
use crate::brute_force::brute_force_caesar;


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
        )
        .arg(
            Arg::with_name("caesar")
                .long("caesar")
                .help("Sets cipher mode to caesar")
        )
        .arg(
            Arg::with_name("affine")
                .long("affine")
                .help("Sets cipher mode to affine")
        )
        .arg(
            Arg::with_name("brute_force")
                .long("brute")
                .help("Runs bruteforce attack on input")
        )
        .arg(
            Arg::with_name("a")
                .short('a')
                .help("Sets variable to affine cipher")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("b")
                .short('b')
                .help("Sets variable to affine cipher")
                .takes_value(true)
        )
        .get_matches();

    let is_encrypt = matches.is_present("encrypt");
    let is_decrypt = matches.is_present("decrypt");
    let is_caesar = matches.is_present("caesar");
    let is_affine = matches.is_present("affine");
    let is_brute_force = matches.is_present("brute_force");
    let mut a = 0;
    let mut b = 0;
    if is_affine {
        a = matches.value_of("a").unwrap().parse::<i32>().expect("variable must be an integer");
        b = matches.value_of("b").unwrap().parse::<i32>().expect("variable must be an integer");
    }
    let mut key: i32 = 0;
    if matches.is_present("key") { key = matches.value_of("key").unwrap().parse::<i32>().expect("Key must be an integer"); }
    let input_file = matches.value_of("input").unwrap();
    let mut output_file = "";
    if matches.is_present("output") { output_file = matches.value_of("output").unwrap(); }

    let input_text = fs::read_to_string(input_file).expect("Error reading input file");
    let mut processed_text = "".parse::<String>().unwrap();
     if !is_brute_force {
         processed_text = if is_encrypt {
        match (is_caesar, is_affine) {
            (true, false) => caesar::cipher(&input_text, key),
            (false, true) => affine::encrypt(&input_text, a, b),
            _ => panic!("No encryption method specified"),
        }
    } else if is_decrypt {
        match (is_caesar, is_affine) {
            (true, false) => caesar::cipher(&input_text, -key),
            (false, true) => {
                match affine::decrypt(&input_text, a, b) {
                    Some(text) => text,
                    None => panic!("Decryption failed due to invalid input"),
                }
            }
            _ => panic!("No decryption method specified")
        }
    } else {
        panic!("Either encrypt or decrypt must be specified");
    }};

    if matches.is_present("output") { fs::write(output_file, processed_text.clone()).expect("Error writing to output file"); }
    if is_brute_force {
        println!("{:?}", brute_force_caesar(&input_text));
    }
    Ok(())
}
