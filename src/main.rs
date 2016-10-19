extern crate crypto_pals;
extern crate rustc_serialize;

use crypto_pals::*;
use rustc_serialize::hex::FromHex;

fn main() {
    // Set 1 Challenge 3
    //let (_, decrypted) = single_byte_cipher::decrypt(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".from_hex().unwrap());
    //println!("{}", String::from_utf8_lossy(&decrypted));

    // Set 1 Challenge 4
    cipher_from_file();
}

fn cipher_from_file() {
    let xors = include_str!("../set1_challenge4.txt");

    let mut score = 0;
    let mut decrypted = Vec::new();

    for line in xors.lines() {
        let (new_score, decrypt_attempt) = single_byte_cipher::decrypt(&line.from_hex().unwrap());

        if new_score > score {
            score = new_score;
            decrypted = decrypt_attempt;
        }
    }
    println!("{}", String::from_utf8_lossy(&decrypted));
}
