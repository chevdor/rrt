use std::fmt::UpperHex;

pub fn dec2hex<T: UpperHex>(x: T, width: usize) -> String {
    format!("{:0width$X}", x, width = width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_dec2hex() {
        assert_eq!(dec2hex(0, 2), "00");
        assert_eq!(dec2hex(0, 4), "0000");
        assert_eq!(dec2hex(10, 2), "0A");
        assert_eq!(dec2hex(15, 2), "0F");
        assert_eq!(dec2hex(16, 2), "10");
        assert_eq!(dec2hex(255, 2), "FF");
        assert_eq!(dec2hex(11041, 5), "02B21");
    }
}
