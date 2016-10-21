pub fn encrypt(key: &[u8], raw: &[u8]) -> Vec<u8> {
    let mut encrypted = Vec::with_capacity(raw.len());

    for bytes in raw.chunks(key.len()) {
        for (raw, key) in bytes.iter().zip(key) {
            encrypted.push(key ^ raw);
        }
    }

    encrypted
}


#[cfg(test)]
mod tests {
    use rustc_serialize::hex::FromHex;
    use super::*;

    #[test]
    fn bigger_test() {
        let string = "When Mr. Bilbo Baggins of Bag End announced that he would shortly be celebrating his eleventy-first birthday with a party of special magnificence, there was much talk and excitement in Hobbiton.";
        // "
        let key = b"BILBO";
        assert_eq!(string, String::from_utf8_lossy(&*encrypt(&key[..], &encrypt(&key[..], string.as_bytes()))));
    }

    #[test]
    fn it_works_decrypting_too() {
        let string = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap();

        let encrypted = encrypt(&b"ICE"[..], &string);

        assert_eq!(String::from_utf8_lossy(&encrypted), "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal");
    }

    #[test]
    fn it_works() {
        let string = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

        let encrypted = encrypt(&b"ICE"[..], string.as_bytes());

        assert_eq!(encrypted, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap());
    }
}
