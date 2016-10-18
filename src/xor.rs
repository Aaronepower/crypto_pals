
pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut xor_vec = Vec::with_capacity(a.len());

    for (a, b) in a.iter().zip(b) {
        xor_vec.push(a ^ b);
    }

    xor_vec
}

#[cfg(test)]
mod tests {
    use super::fixed_xor;
    use rustc_serialize::hex::FromHex;

    #[test]
    fn it_works() {
        let a = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
        let b = "686974207468652062756c6c277320657965".from_hex().unwrap();

        assert_eq!(fixed_xor(&a, &b), "746865206b696420646f6e277420706c6179".from_hex().unwrap())
    }
}
