use std::convert::TryFrom;

fn main() {
    let cleaned = "ABCDEFEXX";
    let found = &cleaned.as_bytes()[SIZE - 2..].try_into();
    let f: &[u8; 2] = found;

    println!("{:?}", f);
}
