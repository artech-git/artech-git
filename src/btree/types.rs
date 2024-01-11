


pub fn parse_varint(buf: &[u8]) -> (u64, usize) {
    let mut result: u64 = 0;
    let mut len: usize = 1;
    for byte in buf.iter().take(8) {
        result <<= 7;
        result += (*byte as u64) & 0b0111_1111;
        if byte & 0b1000_0000 == 0 {
            return (result, len);
        }
        len += 1;
    }
    result <<= 8;
    result += buf[8] as u64;
    (result, 9)

}

#[test]
fn test_parse_varint_0() {
    assert_eq!(parse_varint(b"\x00"), (0, 1))
}

#[test]
fn test_parse_varint_len1() {
    assert_eq!(parse_varint(b"\x7f"), (127, 1))
}

#[test]
fn test_parse_varint_len2() {
    assert_eq!(
        parse_varint(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff"),
        (u64::MAX, 9)
    )
}