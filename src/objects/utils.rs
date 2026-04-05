use std::fmt::Write;

pub(crate) fn bytes_to_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_string() {
        assert_eq!("1a3f", bytes_to_string(&[0x1a, 0x3f]));
    }

    #[test]
    fn test_bytes_to_string_single_byte() {
        assert_eq!("01", bytes_to_string(&[0x1]));
    }

    #[test]
    fn test_bytes_to_string_empty() {
        assert_eq!(bytes_to_string(&[]), "");
    }
}
