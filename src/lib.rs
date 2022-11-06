//! d64-rs is a rustlang crate to create, read and update .D64 files.
//!
//! Although the crate is created for d64 files it could easily be extended
//! to support other track/sector based formats as well.
//!
//! # What is a D64 file?
//!
//! D64 file is a file-format that contains raw data of a Commodore 64 disk.
//! It is used for emulation, development and reservation.
//!
//! The D64 file format is a sequential dump (no headers) of all tracks and
//! sectors of the disk as created by the default ROM of the Commodore 1541
//! and compatible drives.
//!
//! # What about the G64 file?
//!
//! G64 file is not supported by this crate. The G64 compared to the D64 file
//! format wraps sectors with sync headers/footers. Some copy protection and
//! improved disk formats utilized those sync headers.
//!
//! # How to use this crate?
//!
//! ## How to initialize an 1541 formatted D64 file?
//!
//! The d64 crate does everything in memory. [Disk::write_to_path] should
//! be used to create/update the file.
//!
//! ```
//! use d64::*;
//! use std::path::Path;
//!
//! let mut disk = Disk::<Commodore1541>::new();
//! disk.format();
//! disk.set_name(&String::from("VOLUME 1"));
//!
//! disk.write_to_path(&Path::new("volume1.d64")).unwrap();
//! ```
mod commodore1541;
mod disk;
mod layout;
mod petscii;
mod sector;
mod track;

pub use commodore1541::*;
pub use disk::*;
pub use layout::*;
pub use petscii::*;
pub use sector::*;
pub use track::*;
