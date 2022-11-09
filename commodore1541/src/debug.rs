use d64::Sector;
use petscii::decode_petscii;

pub fn print_sector(sector: &Sector) {
    let mut x = 0;
    let mut decoded = String::new();
    for a in sector.as_slice() {
        print!("{:02x} ", a);
        decoded.push(decode_petscii(*a) as char);
        x += 1;
        if x == 16 {
            x = 0;
            println!("  {}", decoded);
            decoded.clear();
        }
    }
}
