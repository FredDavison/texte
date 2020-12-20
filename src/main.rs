use std::io;
use std::io::Read;

fn main() {
    let mut buf = [0; 1];
    let mut i = 0;

    loop {

        let s = io::stdin();
        let mut handle = s.take(1);  // returns a Read instance
        let n = handle.read(&mut buf).expect("Error reading");

        let char = buf.bytes().next().expect("").expect("");
        println!("{}: {:?}, {} bytes", i, char, n);

        if char == 113 {
            break;
        };
        i += 1;
    }
}
