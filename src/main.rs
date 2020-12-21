use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();
    let mut _i = 0;

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        if b == 113 {
            break;
        };

        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        _i += 1;
    }
}

}

