
pub fn encrypt(key: &[u8], raw: &[u8]) -> Vec<u8> {
    let mut encrypted = Vec::with_capacity(raw.len());

    for bytes in raw.chunks(key.len()) {
        for (raw, key) in key.iter().zip(bytes) {
            encrypted.push(raw ^ key);
        }
    }
    encrypted
}


#[cfg(test)]
mod tests {
    use rustc_serialize::hex::FromHex;
    use super::*;

    #[test]
    fn it_works() {
        let string = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";

        let encrypted = encrypt(&b"ICE"[..], string.as_bytes());

        assert_eq!(encrypted, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".from_hex().unwrap());
    }
}
