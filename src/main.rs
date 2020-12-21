use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;

                if b == to_ctrl_byte('q') {
                    break;
                };

                if c.is_control() {
                    println!("{:?} {:#b}\r", b, b);
                } else {
                    println!("{:?} {:#b} ({})\r", b, b, c);
                }
            }
            Err(e) => die(e),
        }
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}
