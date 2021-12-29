use byteorder::*;

fn main() {
    let mut b: Vec<u8> = vec![0; 4];
    LittleEndian::write_u16(&mut b, 0x1234u16);
    LittleEndian::write_u16(&mut b[2..], 0xcdefu16);

    for i in b.iter().copied() {
        print!("{:#x} ", i);
    }
    println!();
    println!("{:#x}", LittleEndian::read_u16(&b[0..2]));
    println!("{:#x}", LittleEndian::read_u16(&b[2..4]));
}
