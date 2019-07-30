use crc16::{State, XMODEM};
use std::env;

pub fn generate_crc16(pktdata: &[u8]) -> u16 {
    State::<XMODEM>::calculate(&pktdata)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut bytes: Vec<u8> = Vec::new();

    for arg in args.iter().skip(1) {
        let numbers = arg.split(",");

        for number in numbers {
            if !number.is_empty() {
                match number.parse::<u8>() {
                    Ok(value) => bytes.push(value),
                    Err(_) => println!("Not u8, won't be added: {}", number),
                }
            }
        }
    }

    bytes.push(23);

    let crc = generate_crc16(&bytes);
    let crc1 = crc / 256;
    let crc2 = crc % 256;

    println!("{},{}", crc1, crc2);
}
