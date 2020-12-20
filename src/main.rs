use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use termios::*;

fn main() {
    let mut buf = [0; 1];
    let mut i = 0;
    enable_raw_mode();

    loop {
        let s = io::stdin();
        let mut handle = s.take(1); // returns a Read instance
        let n = handle.read(&mut buf).expect("Error reading");

        let char = buf.bytes().next().expect("").expect("");
        println!("{}: {:?}, {} bytes", i, char, n);

        if char == 113 {
            break;
        };
        i += 1;
    }
}

fn enable_raw_mode() {
    let fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd).expect("");

    termios.c_lflag &= !ECHO;
    tcsetattr(fd, TCSAFLUSH, &termios).expect("Error setting termios constants.");
}
