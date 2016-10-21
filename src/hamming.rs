pub fn distance(a: &[u8], b: &[u8]) -> usize {
    let mut distance:usize = 0;

    for (a, b) in a.iter().zip(b) {
        distance += weight(a ^ b);
    }
    distance
}

pub fn weight(mut x: u8) -> usize {
    let mut count = 0;

    while x > 0 {
        count += 1;
        x &= x - 1
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(distance(&b"this is a test"[..], &b"wokka wokka!!!"[..]), 37);
    }
}
