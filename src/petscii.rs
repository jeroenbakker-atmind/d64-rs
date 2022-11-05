pub const PETSCII_NBSP: u8 = 160;
pub const PETSCII_A: u8 = 65;

/// Decode an petscii byte to ascii char.
///
/// # Example
/// ```
/// use d64::*;
/// assert_eq!(decode_petscii(PETSCII_A), 'A');
/// ```
pub fn decode_petscii(petscii: u8) -> char {
    //         1         2         3         4         5         6
    "................................ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[.].................................................................. ...............................................................................................".as_bytes()[petscii as usize] as char
}

/// Encode an ascii char to petscii.
///
/// # Example
/// ```
/// use d64::*;
/// assert_eq!(encode_petscii('A', PETSCII_NBSP), PETSCII_A);
/// ```
pub fn encode_petscii(ascii: char, default: u8) -> u8 {
    if ascii >= 'A' && ascii <= 'Z' {
        return (ascii as u8 - 'A' as u8) + PETSCII_A;
    }
    return default;
}
