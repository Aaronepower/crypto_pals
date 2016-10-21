use xor;

pub fn decrypt(encrypted: &[u8]) -> (usize, u8, Vec<u8>) {
    const CHARS: &'static [u8; 52] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let encrypted_length = encrypted.len();
    let mut score = 0;
    let mut key = 0;
    let mut decrypted = Vec::new();
    for &ch in CHARS.iter() {
        let char_vec: Vec<u8> = vec![ch; encrypted_length];
        let decrypt_attempt = xor::fixed_xor(&encrypted, &char_vec);

        let new_score = decrypt_attempt.iter().filter(|&&x| x == b'e' || x == b't' || x == b'i' || x== b'o' || x == b'n' || x == b's' || x == b' ').count();

        if new_score > score {
            score = new_score;
            key = ch;
            decrypted = decrypt_attempt;
        }
    }
    (score, key, decrypted)
}

