use crate::checksum::checksum::RRTChecksum;
use fletcher::generic_fletcher::Fletcher;
use std::fmt::Debug;

type Output = [u8; 2];
type Fletcher16 = Fletcher<u16, u8>;

pub struct ChecksumV01 {
    fletcher: Fletcher16,
    checksum: Option<Output>,
}

impl Debug for ChecksumV01 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.checksum)
    }
}

impl ChecksumV01 {
    pub fn new() -> Self {
        Self {
            checksum: None,
            fletcher: fletcher::Fletcher16::new(),
        }
    }
}

impl RRTChecksum for ChecksumV01 {
    // fn calculate(s: &str) -> Output {
    //     let content = s.as_bytes();
    //     Self::calculate(content)
    // }

    /// We want a checksum being a value 65..90
    /// So we take the modulo 26 and shift to the first char.  
    fn calculate(&mut self, data: &[u8]) -> [u8; 2] {
        self.fletcher.update(&data);

        let v_u16 = self.fletcher.value();
        [(v_u16 >> 8) as u8, v_u16 as u8]
    }

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
        let mut checksum = ChecksumV01::new();

        assert_eq!(checksum.calculate("A".as_bytes()), [1, 2]);
    }

    #[test]
    fn it_calculate() {
        let mut checksum = ChecksumV01::new();

        assert_eq!(checksum.calculate(b"A"), [1, 2]);
    }

    #[test]
    fn it_calculates_from_str() {
        let mut checksum = ChecksumV01::new();

        // TODO: Fix that, all the checksums should be 65..=90
        assert_eq!(checksum.calculate(b"0"), [1, 2]);
        assert_eq!(checksum.calculate(b"1"), [1, 2]);
        assert_eq!(checksum.calculate(b"A"), [1, 2]);
        assert_eq!(checksum.calculate(b"AA"), [1, 2]);
        assert_eq!(checksum.calculate(b"AB09"), [1, 2]);
        assert_eq!(checksum.calculate(b"ZZZZZZ"), [1, 2]);
        assert_eq!(checksum.calculate(b"010012345TWBABAEFGH"), [1, 2]);
    }

    #[test]
    fn it_has_a_checksum_always_between_65_and_90() {
        let mut checksum = ChecksumV01::new();

        let mut s = String::new();

        for _ in 1..26 * 100 {
            s += "A";
            // println!("{}", s);
            let c = checksum.calculate(s.as_bytes());
            assert!(c[0] >= 65); // Ascii A
            assert!(c[1] >= 65); // Ascii A
            assert!(c[0] <= 90); // Ascii Z
            assert!(c[1] <= 90); // Ascii Z
        }
    }

    #[test]
    fn it_prevents_typical_swaps() {
        let mut checksum = ChecksumV01::new();

        assert!(checksum.calculate(b"AB") != checksum.calculate(b"BA"));
        // assert!(
        //     checksum.calculate("010012345TWBABAEFGH")
        //         != checksum.calculate("100012345TWBABAEFGH")
        // );
    }
}
