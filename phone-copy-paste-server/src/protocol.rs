/// Encode a text message for the wire protocol.
///
/// Format: 4-byte big-endian length prefix followed by UTF-8 bytes.
pub fn encode_message(text: &str) -> Vec<u8> {
    let bytes = text.as_bytes();
    let len = bytes.len() as u32;
    let mut buf = Vec::with_capacity(4 + bytes.len());
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(bytes);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty_string() {
        assert_eq!(encode_message(""), vec![0, 0, 0, 0]);
    }

    #[test]
    fn encode_ascii() {
        let encoded = encode_message("hello");
        let mut expected = vec![0, 0, 0, 5];
        expected.extend_from_slice(b"hello");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_multibyte_utf8() {
        let text = "café"; // 'é' is 2 bytes in UTF-8
        let encoded = encode_message(text);
        let raw = text.as_bytes();
        assert_eq!(raw.len(), 5); // 3 ASCII + 2 for é
        assert_eq!(&encoded[..4], &5u32.to_be_bytes());
        assert_eq!(&encoded[4..], raw);
    }

    #[test]
    fn length_prefix_is_big_endian() {
        // 256 bytes of 'a'
        let text: String = "a".repeat(256);
        let encoded = encode_message(&text);
        assert_eq!(&encoded[..4], &[0, 0, 1, 0]); // 256 in BE
        assert_eq!(encoded.len(), 4 + 256);
    }
}
