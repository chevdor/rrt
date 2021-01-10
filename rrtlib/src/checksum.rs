pub struct Checksum;

pub trait Calculate {
    fn calculate(s: &str) -> u8;
    fn calculates_from_bytes(s: &[u8]) -> u8;
}

pub trait Verify {
    fn verify(data: [u8], checksum: u8) -> bool;
}

impl Calculate for Checksum {
    fn calculate(s: &str) -> u8 {
        let content = s.as_bytes();
        Self::calculates_from_bytes(content)
    }

    /// We want a checksum being a value 65..90
    /// So we take the modulo 26 and shift to the first char.  
    fn calculates_from_bytes(s: &[u8]) -> u8 {
        let sum = s.iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        (sum % 26) + 65
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates() {
        assert_eq!(Checksum::calculate("A"), 78);
    }

    #[test]
    fn it_calculates_from_bytes() {
        assert_eq!(Checksum::calculates_from_bytes(b"A"), 78);
    }

    #[test]
    fn it_calculates_from_str() {
        // TODO: Fix that, all the checksums should be 65..=90
        assert_eq!(Checksum::calculates_from_bytes(b"0"), 87);
        assert_eq!(Checksum::calculates_from_bytes(b"1"), 88);
        assert_eq!(Checksum::calculates_from_bytes(b"A"), 78);
        assert_eq!(Checksum::calculates_from_bytes(b"AA"), 65);
        assert_eq!(Checksum::calculates_from_bytes(b"AB09"), 67);
        assert_eq!(Checksum::calculates_from_bytes(b"ZZZZZZ"), 67);
        assert_eq!(Checksum::calculates_from_bytes(b"010012345TWBABAEFGH"), 74);
    }

    #[test]
    fn it_has_a_checksum_always_between_65_and_90() {
        // TODO: Fix that, all the checksums should be 65..=90
        let mut s = String::new();

        for _ in 1..26 * 100 {
            s += "A";
            // println!("{}", s);
            let checksum = Checksum::calculates_from_bytes(s.as_bytes());
            assert!(checksum >= 65); // Ascii A
            assert!(checksum <= 90); // Ascii Z
        }
    }
}
