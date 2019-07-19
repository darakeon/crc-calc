use crc16::{State, XMODEM};

pub fn generate_crc16(pktdata: &[u8]) -> u16 {
    State::<XMODEM>::calculate(&pktdata)
}

fn main() {
	let bytes = [67, 76, 79, 48, 52, 48, 23];
    
	let crc = generate_crc16(&bytes);
	let crc1 = crc / 256;
	let crc2 = crc % 256;
	
	println!("{},{}", crc1, crc2);
}
