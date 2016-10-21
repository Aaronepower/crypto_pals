use hamming;
use single_byte_cipher;
use repeating_xor;

use std::usize;

pub fn decrypt(input: &[u8]) -> Vec<u8> {

    let mut keysizes = [usize::MAX; 3];
    let mut keysizes_distance = [usize::MAX; 3];

    for x in 2..41 {

        let ref first = input[0..x+1];
        let ref second = input[x+1..(x * 2) + 1];

        let distance = hamming::distance(&first, &second) / x;

        for (ref mut key, ref mut key_distance) in keysizes.iter_mut().zip(&mut keysizes_distance) {
            if distance < **key_distance {
                **key = x;
                **key_distance = distance;
                break;
            }
        }
    }

    let mut previous_score = 0;
    let mut key = Vec::new();

    for keysize in &keysizes {
        let mut key_blocks: Vec<Vec<u8>> = vec![vec![]; *keysize];

        for block in input.chunks(*keysize) {
            for (byte, mut block) in block.iter().zip(&mut key_blocks) {
                block.push(*byte);
            }
        }

        let mut decrypted = Vec::new();
        let mut overall_score = 0;

        for keys in key_blocks {
            let (score, key, _) = single_byte_cipher::decrypt(&keys);
            decrypted.push(key);
            overall_score += score;
        }

        overall_score /= *keysize;

        if overall_score > previous_score {
            key = decrypted;
            previous_score = overall_score;
        }
    }

    repeating_xor::encrypt("BILBO".as_bytes(), &input)
}


#[cfg(test)]
mod tests {
    use super::*;
    use base64;
    use repeating_xor;

    #[test]
    fn it_works() {
        let message = "When Mr. Bilbo Baggins of Bag End announced that he would shortly be celebrating".as_bytes();
        // "
        let key = "BILBO".as_bytes();
        let xor_encrypted = repeating_xor::encrypt(key, message);
        let base64 = base64::to_base64(&xor_encrypted);
        let unbase64 = base64::from_base64(&base64);
        let decrypted = repeating_xor::encrypt(key, &unbase64);

        assert_eq!(message, encrypt(key, xor_encrypted));
        assert_eq!(base64::from_base64(&base64), &*unbase64);
        assert_eq!(decrypted, repeating_xor::encrypt(key, &unbase64));

        for (index, (a, b)) in message.iter().zip(&repeating_xor::encrypt(key, &unbase64)).enumerate() {
            println!("CHARACTERS AT INDEX [{:3}, {:3}]", a, b);
            assert_eq!(a, b);
        }
    }
}
