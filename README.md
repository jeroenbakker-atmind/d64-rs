# d64-rs

Rust crate to read and manipulate C64 disk images.

Motivation is to create disk images from modern machines without using special programs.
Although development target Commodore 1541, it can be extended to support
other layouts as well.

Still in early development.

* [x] Reading d64 file.
* [x] Writing d64 file.
* [x] Rename disk name.
* [x] Format disk.
* [x] List entries.
* [x] Read file.
* [x] Create file.
* [x] Delete file.
* [ ] Write file.
* [x] Disk IDs. (01-2A signature)

Note after some development I came across https://docs.rs/cbm/0.1.0/cbm/ which has all the features
that I required. This project is only for my own research. I would advice to use cbm and focus
development to a single crate.