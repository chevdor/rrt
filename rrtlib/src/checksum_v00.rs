use crate::checksum::RRTChecksum;

// type Output = u8; // TODO: I would prefer this to be used instead of the next
type Output = [u8];

#[derive(Debug)]
pub struct ChecksumV00 {
    checksum: Option<u8>,
}

impl ChecksumV00 {
    pub fn new() -> Self {
        Self { checksum: None }
    }
}

impl RRTChecksum for ChecksumV00 {
    // fn calculate(s: &str) -> u8 {
    //     let content = s.as_bytes();
    //     Self::calculate(content)
    // }

    /// We want a checksum being a value 65..90
    /// So we take the modulo 26 and shift to the first char.
    fn calculate(&self, s: &[u8]) -> Vec<u8> {
        let sum: u8 = s.iter().fold(0, |s, &x| s.wrapping_add(x));

        // let sum: u8 = indexes
        //     .zip(s.iter())
        //     .fold(0, |acc, (i, &x)| acc.wrapping_add(x * u8::MAX * i));

        vec![(sum % 26) + 65]
    }

    // fn calculate<T>(s: T) -> u8 {
    //     todo!()
    // }

    fn verify(&self, _: &[u8], _: u8) -> bool {
        todo!()
    }

    fn is_valid(&self, _: &[u8]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates() {
        let checksum = ChecksumV00::new();
        assert_eq!(checksum.calculate("A".as_bytes()), [78]);
    }

    #[test]
    fn it_calculate() {
        let checksum = ChecksumV00::new();
        assert_eq!(checksum.calculate(b"A"), [78]);
    }

    #[test]
    fn it_calculates_from_str() {
        let checksum = ChecksumV00::new();

        // TODO: Fix that, all the checksums should be 65..=90
        assert_eq!(checksum.calculate(b"0"), [87]);
        assert_eq!(checksum.calculate(b"1"), [88]);
        assert_eq!(checksum.calculate(b"A"), [78]);
        assert_eq!(checksum.calculate(b"AA"), [65]);
        assert_eq!(checksum.calculate(b"AB09"), [67]);
        assert_eq!(checksum.calculate(b"ZZZZZZ"), [67]);
        assert_eq!(checksum.calculate(b"010012345TWBABAEFGH"), [74]);
    }

    #[test]
    fn it_has_a_checksum_always_between_65_and_90() {
        let checksum = ChecksumV00::new();
        let mut s = String::new();

        for _ in 1..26 * 100 {
            s += "A";
            // println!("{}", s);
            let c = checksum.calculate(s.as_bytes())[0];
            assert!(c >= 65); // Ascii A
            assert!(c <= 90); // Ascii Z
        }
    }

    #[test]
    #[ignore = "Does not work for this checksum"]
    fn it_prevents_typical_swaps() {
        let checksum = ChecksumV00::new();

        assert!(checksum.calculate("AB".as_bytes()) != checksum.calculate("BA".as_bytes()));
        // assert!(
        //     checksum.calculate("010012345TWBABAEFGH")
        //         != checksum.calculate("100012345TWBABAEFGH")
        // );
    }
}
