pub const PETSCII_ZERO: u8 = 48;
pub const PETSCII_ONE: u8 = 49;
pub const PETSCII_TWO: u8 = 50;
pub const PETSCII_THREE: u8 = 51;
pub const PETSCII_FOUR: u8 = 52;
pub const PETSCII_FIVE: u8 = 53;
pub const PETSCII_SIX: u8 = 54;
pub const PETSCII_SEVEN: u8 = 55;
pub const PETSCII_EIGHT: u8 = 56;
pub const PETSCII_NINE: u8 = 57;

pub const PETSCII_A: u8 = 65;
pub const PETSCII_B: u8 = 66;
pub const PETSCII_C: u8 = 67;
pub const PETSCII_D: u8 = 68;
pub const PETSCII_E: u8 = 69;
pub const PETSCII_F: u8 = 70;
pub const PETSCII_G: u8 = 71;
pub const PETSCII_H: u8 = 72;
pub const PETSCII_I: u8 = 73;
pub const PETSCII_J: u8 = 74;
pub const PETSCII_K: u8 = 75;
pub const PETSCII_L: u8 = 76;
pub const PETSCII_M: u8 = 77;
pub const PETSCII_N: u8 = 78;
pub const PETSCII_O: u8 = 79;
pub const PETSCII_P: u8 = 80;
pub const PETSCII_Q: u8 = 81;
pub const PETSCII_R: u8 = 82;
pub const PETSCII_S: u8 = 83;
pub const PETSCII_T: u8 = 84;
pub const PETSCII_U: u8 = 85;
pub const PETSCII_V: u8 = 86;
pub const PETSCII_W: u8 = 87;
pub const PETSCII_X: u8 = 88;
pub const PETSCII_Y: u8 = 89;
pub const PETSCII_Z: u8 = 90;

pub const PETSCII_NBSP: u8 = 160;

/// Decode an petscii byte to ascii char.
///
/// # Example
/// ```
/// use d64::*;
/// assert_eq!(decode_petscii(PETSCII_A), 'A');
/// assert_eq!(decode_petscii(PETSCII_J), 'J');
/// assert_eq!(decode_petscii(PETSCII_Z), 'Z');
/// assert_eq!(decode_petscii(PETSCII_ZERO), '0');
/// assert_eq!(decode_petscii(PETSCII_ONE), '1');
/// assert_eq!(decode_petscii(PETSCII_TWO), '2');
/// assert_eq!(decode_petscii(PETSCII_THREE), '3');
/// assert_eq!(decode_petscii(PETSCII_FOUR), '4');
/// assert_eq!(decode_petscii(PETSCII_FIVE), '5');
/// assert_eq!(decode_petscii(PETSCII_SIX), '6');
/// assert_eq!(decode_petscii(PETSCII_SEVEN), '7');
/// assert_eq!(decode_petscii(PETSCII_EIGHT), '8');
/// assert_eq!(decode_petscii(PETSCII_NINE), '9');
/// ```
pub fn decode_petscii(petscii: u8) -> char {
    "................................ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[.].................................................................. ...............................................................................................".as_bytes()[petscii as usize] as char
}

/// Encode an ascii char to petscii.
///
/// Lower case characters are automatically converted to upper case PETSCII.
///
/// # Example
/// ```
/// use d64::*;
/// assert_eq!(encode_petscii('A', PETSCII_NBSP), PETSCII_A);
/// assert_eq!(encode_petscii('a', PETSCII_NBSP), PETSCII_A);
/// assert_eq!(encode_petscii('I', PETSCII_NBSP), PETSCII_I);
/// assert_eq!(encode_petscii('Z', PETSCII_NBSP), PETSCII_Z);
/// assert_eq!(encode_petscii('0', PETSCII_NBSP), PETSCII_ZERO);
/// assert_eq!(encode_petscii('1', PETSCII_NBSP), PETSCII_ONE);
/// assert_eq!(encode_petscii('2', PETSCII_NBSP), PETSCII_TWO);
/// assert_eq!(encode_petscii('3', PETSCII_NBSP), PETSCII_THREE);
/// assert_eq!(encode_petscii('4', PETSCII_NBSP), PETSCII_FOUR);
/// assert_eq!(encode_petscii('5', PETSCII_NBSP), PETSCII_FIVE);
/// assert_eq!(encode_petscii('6', PETSCII_NBSP), PETSCII_SIX);
/// assert_eq!(encode_petscii('7', PETSCII_NBSP), PETSCII_SEVEN);
/// assert_eq!(encode_petscii('8', PETSCII_NBSP), PETSCII_EIGHT);
/// assert_eq!(encode_petscii('9', PETSCII_NBSP), PETSCII_NINE);
/// ```
pub fn encode_petscii(ascii: char, default: u8) -> u8 {
    if ('A'..='Z').contains(&ascii) {
        return (ascii as u8 - b'A') + PETSCII_A;
    }
    if ('a'..='z').contains(&ascii) {
        return (ascii as u8 - b'a') + PETSCII_A;
    }
    if ('0'..='9').contains(&ascii) {
        return (ascii as u8 - b'0') + PETSCII_ZERO;
    }
    default
}

#[derive(Debug)]
pub struct PetsciiString {
    bytes: Vec<u8>,
}

impl PetsciiString {
    pub fn fixed_size(bytes: &[u8]) -> PetsciiString {
        let b = Vec::from(bytes);
        PetsciiString { bytes: b }
    }
}

impl From<&String> for PetsciiString {
    fn from(src: &String) -> PetsciiString {
        let mut result = PetsciiString {
            bytes: Vec::with_capacity(src.len()),
        };
        for ch in src.chars() {
            result.bytes.push(encode_petscii(ch, PETSCII_NBSP));
        }
        result
    }
}

impl From<&PetsciiString> for String {
    fn from(src: &PetsciiString) -> String {
        let mut result = String::new();
        for byte in &src.bytes {
            if *byte == PETSCII_NBSP {
                break;
            }
            let ch = decode_petscii(*byte);
            result.push(ch);
        }
        result
    }
}
