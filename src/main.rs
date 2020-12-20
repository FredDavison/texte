use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use termios::*;

fn main() {
    let mut buf = [0; 1];
    let mut i = 0;

    let fd = io::stdin().as_raw_fd();
    let original_term = Termios::from_fd(fd).expect("");
    let term = Termios::from_fd(fd).expect("");
    enable_raw_mode(fd, term);

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
    disable_raw_mode(fd, original_term);
}

fn enable_raw_mode(fd: i32, mut termios: Termios) {
    termios.c_lflag &= !(ECHO | ICANON);
    tcsetattr(fd, TCSAFLUSH, &termios).expect("Error updating termios constants.");
}

fn disable_raw_mode(fd: i32, original_term: Termios) {
    tcsetattr(fd, TCSAFLUSH, &original_term).expect("Error resetting termios constants to default.");
}
