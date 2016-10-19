use rustc_serialize::hex::FromHex;
use xor;

pub fn decrypt(encrypted: &[u8]) -> (usize, Vec<u8>){
    const CHARS: &'static [u8; 62] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let encrypted_length = encrypted.len();
    let mut score = 0;
    let mut decrypted = Vec::new();
    for &ch in CHARS.iter() {
        let char_vec: Vec<u8> = vec![ch; encrypted_length];
        let decrypt_attempt = xor::fixed_xor(&encrypted, &char_vec);

        let new_score = decrypt_attempt.iter().filter(|&&x| x == b'a' || x == b'e' || x == b'i' || x== b'u' || x == b' ').count();

        if new_score > score {
            score = new_score;
            decrypted = decrypt_attempt;
        }
    }
    (score, decrypted)
}

