use crate::checksum::checksum::Checksum;
use std::fmt::Debug;

type Output = [u8; 2];

/// This second version, based on fletcher16 is much better than the v00 version.
pub struct ChecksumV01 {
    checksum: Option<Output>,
}

impl Debug for ChecksumV01 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.checksum)
    }
}

impl ChecksumV01 {
    pub fn new() -> Self {
        Self { checksum: None }
    }
}

impl Checksum<Output> for ChecksumV01 {
    /// We want a checksum being a value 65..90
    /// So we take the modulo 26 and shift to the first char.  
    fn calculate(&self, data: &[u8]) -> [u8; 2] {
        let mut fletcher = fletcher::Fletcher16::new();
        fletcher.update(data);
        let v_u16 = fletcher.value();
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
        let checksum = ChecksumV01::new();

        assert_eq!(checksum.calculate("A".as_bytes()), [78, 78]);
    }

    #[test]
    fn it_calculates_from_str() {
        let checksum = ChecksumV01::new();
        assert_eq!(checksum.calculate(b"0"), [87, 77]);
        assert_eq!(checksum.calculate(b"1"), [88, 74]);
        assert_eq!(checksum.calculate(b"AA"), [78, 65]);
        assert_eq!(checksum.calculate(b"AB09"), [88, 79]);
        assert_eq!(checksum.calculate(b"ZZZZZZ"), [66, 65]);
        assert_eq!(checksum.calculate(b"010012345TWBABAEFGH"), [86, 72]);
    }

    #[test]
    fn it_has_deterministic_checksum() {
        let checksum = ChecksumV01::new();
        let token = "1101000012346TWRAJQFIZWRP";
        let r1 = checksum.calculate(token.as_bytes());
        let r2 = checksum.calculate(token.as_bytes());
        assert_eq!(r1, r2);
    }

    #[test]
    fn it_has_a_checksum_always_between_65_and_90() {
        let checksum = ChecksumV01::new();

        let mut s = String::new();

        for _ in 1..26 * 100 {
            s += "A";
            let c = checksum.calculate(s.as_bytes());
            assert!(c[0] >= 65); // Ascii A
            assert!(c[1] >= 65); // Ascii A
            assert!(c[0] <= 90); // Ascii Z
            assert!(c[1] <= 90); // Ascii Z
        }
    }

    #[test]
    fn it_prevents_typical_swaps() {
        let checksum = ChecksumV01::new();

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
