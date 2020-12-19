use std::io;
use std::io::Read;

fn main() {
    let mut buf: Vec<u8> = Vec::new();

    loop {
        let n = io::stdin().read(&mut buf) 
            .ok()
            .expect("error");
        println!("{:?}", n);
    }
}
