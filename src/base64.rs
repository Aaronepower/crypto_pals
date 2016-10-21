static CHARS: &'static [u8; 65] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

const TWO_BITS:u8 = 0b11;
const FOUR_BITS:u8 = 0b1111;
const SIX_BITS:u8 = 0b111111;

pub fn from_base64(encoded: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();

    if encoded.len() % 4 != 0 {
        panic!("Not valid base64: {:?}", String::from_utf8_lossy(encoded));
    }

    for bytes in encoded.chunks(4) {
        let first  = find(bytes[0]);
        let second = find(bytes[1]);
        let third  = find(bytes[2]);
        let fourth = find(bytes[3]);

        decoded.push((first << 2) | (second >> 4));

        if third != 64 {
            decoded.push(((second & FOUR_BITS) << 4) | (third >> 2));

            if fourth != 64 {
                decoded.push(((third & TWO_BITS) << 6) | (fourth & SIX_BITS));
            }
        }
    }

    decoded
}

fn find(value: u8) -> u8 {
    for (index, ch) in CHARS[..].iter().enumerate() {
        if value == *ch {
            return index as u8
        }
    }
    64

}

pub fn to_base64(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity((bytes.len() * 4) / 3);

    for group in bytes.chunks(3) {
        let first = group[0];
        let second = if group.len() >= 2 {group[1]} else {0};
        let third = if group.len() == 3 {group[2]} else {0};

        push_encoded(&mut encoded, (first >> 2) & SIX_BITS);
        push_encoded(&mut encoded, ((first & TWO_BITS) << 4) | second >> 4);
        push_encoded(&mut encoded, ((second & FOUR_BITS) << 2) | third >> 6);
        push_encoded(&mut encoded, third & SIX_BITS);
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
mod decoded_tests {
    use super::*;

    #[test]
    fn no_byte_corruption() {
        let original = b"When Mr. Bilbo Baggins of Bag End announced that he would shortly be celebrating his eleventy-first birthday with a party of special magnificence, there was much talk and excitement in Hobbiton.";
        // "

        assert_eq!(&original[..], &*from_base64(&to_base64(&original[..])));
    }

    #[test]
    fn one_character() {
        let vec = b"TQ==";
        let base64 = from_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "M")
    }
    #[test]
    fn two_characters() {
        let vec = b"TWE=";
        let base64 = from_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "Ma")
    }
    #[test]
    fn simple() {
        let vec = b"TWFu";
        let base64 = from_base64(&vec[..]);

        assert_eq!(String::from_utf8_lossy(&*base64), "Man")
    }
    #[test]
    fn it_works() {
        use rustc_serialize::hex::*;
        let vec = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let base64 = from_base64(vec.as_bytes());

        assert_eq!(base64.to_hex(), "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    }
}

#[cfg(test)]
mod encoded_tests {
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

