static CHARS: &'static [u8; 65] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

const TWO_BITS:u8 = 0b11;
const FOUR_BITS:u8 = 0b1111;
const SIX_BITS:u8 = 0b111111;

pub fn to_base64(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity((bytes.len() * 4) / 3);

    for group in bytes.chunks(3) {
        let first = group[0];
        let second = if group.len() >= 2 {group[1]} else {0};
        let third = if group.len() == 3 {group[2]} else {0};

        let first_two = first & TWO_BITS;
        let first_six = (first >> 2) & SIX_BITS;
        push_encoded(&mut encoded, first_six);

        let second_first_four = ( second >> 4 ) & FOUR_BITS;
        let second_six = (first_two << 4) | second_first_four;
        push_encoded(&mut encoded, second_six);

        let second_last_four = second & FOUR_BITS;
        let third_two = third >> 6;
        let third_six = ( second_last_four << 2 ) | third_two;
        push_encoded(&mut encoded, third_six);

        let fourth_six = third & SIX_BITS;
        push_encoded(&mut encoded, fourth_six);
    }
    encoded
}

fn push_encoded(vec: &mut Vec<u8>, byte: u8) {
    if byte != 0 {
        vec.push(CHARS[byte as usize])
    } else {
        vec.push(b'=');
    }
}

#[cfg(test)]
mod tests {
    use super::to_base64;
    #[test]
    fn one_character() {
        let vec = b"M";
        let base64 = to_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "TQ==")
    }
    #[test]
    fn two_characters() {
        let vec = b"Ma";
        let base64 = to_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "TWE=")
    }
    #[test]
    fn simple() {
        let vec = b"Man";
        let base64 = to_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "TWFu")
    }
    #[test]
    fn it_works() {
        use rustc_serialize::hex::FromHex;
        let vec = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".from_hex().unwrap();
        let base64 = to_base64(&vec);

        assert_eq!(String::from_utf8_lossy(&*base64), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }
}

