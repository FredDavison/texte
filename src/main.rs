use std::io;
use std::io::Read;

fn main() {
    let mut buf = [0; 8];

    loop {
        let _n = io::stdin().read(&mut buf) 
            .ok()
            .expect("error");
        let char = buf.bytes().next().expect("no bytes read");
        println!("{:?}", char);
        println!("{}", char == 113);
    }
}
