use crate::checksum::checksum::Checksum;
use fletcher::generic_fletcher::Fletcher;
use std::fmt::Debug;

type Output = [u8; 2];
type Fletcher16 = Fletcher<u16, u8>;

/// This second version, based on fletcher16 is much better than the v00 version.
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

impl Checksum<Output> for ChecksumV01 {
    /// We want a checksum being a value 65..90
    /// So we take the modulo 26 and shift to the first char.  
    fn calculate(&mut self, data: &[u8]) -> [u8; 2] {
        // let mut dat = data.clone();
        self.fletcher.update(&data);
        let v_u16 = self.fletcher.value();
        let to_ascii_caps = |i| i % 26 + 65;
        [to_ascii_caps(v_u16 >> 8) as u8, to_ascii_caps(v_u16) as u8]
    }

    /// Getter
    fn checksum(&self) -> Option<Output> {
        self.checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates() {
        let mut checksum = ChecksumV01::new();

        assert_eq!(checksum.calculate("A".as_bytes()), [78, 78]);
    }

    #[test]
    fn it_calculates_from_str() {
        let mut checksum = ChecksumV01::new();

        // TODO: Fix that, all the checksums should be 65..=90
        assert_eq!(checksum.calculate(b"0"), [87, 77]);
        assert_eq!(checksum.calculate(b"1"), [80, 76]);
        assert_eq!(checksum.calculate(b"AA"), [89, 66]);
        assert_eq!(checksum.calculate(b"AB09"), [78, 65]);
        assert_eq!(checksum.calculate(b"ZZZZZZ"), [78, 69]);
        assert_eq!(checksum.calculate(b"010012345TWBABAEFGH"), [88, 73]);
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

        let set1 = [[b"01", b"10"], [b"AB", b"BA"], [b"0Z", b"Z0"]];
        for sample in &set1 {
            assert!(checksum.calculate(sample[0]) != checksum.calculate(sample[1]));
        }

        let set2 = [
            [b"_01_0012345TWBABAEFGH", b"_10_0012345TWBABAEFGH"],
            [b"010012345_TW_BABAEFGH", b"010012345_WT_BABAEFGH"],
        ];
        for sample in &set2 {
            assert!(checksum.calculate(sample[0]) != checksum.calculate(sample[1]));
        }
    }
}
