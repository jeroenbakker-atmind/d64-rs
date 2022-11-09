//! # What is petscii?
//!
//! PETSCII (PET Standard Code of Information Interchange), also known as CBM ASCII,
//! is the character set used in Commodore 8 bits computers including the famous C64.
//! See <https://en.wikipedia.org/wiki/PETSCII> for more details.
//!
//!
//! # What can this crate do?
//!
//! Current state of this library is still early in development. For now some characters
//! can be converted. But a lot of cases are not supported or even a clear API on how
//! to add support for them.
//!
//! [ ] Petscii has a lower case and upper case concept that they call shifted. Current
//!     not supported.
//!

mod petscii;

pub use self::petscii::*;
