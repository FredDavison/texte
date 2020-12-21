use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use termios::*;

fn main() {
    let mut _i = 0;

    let fd = io::stdin().as_raw_fd();
    let original_term = Termios::from_fd(fd).expect("");
    let term = Termios::from_fd(fd).expect("");
    enable_raw_mode(fd, term);

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
    disable_raw_mode(fd, original_term);
}

fn enable_raw_mode(fd: i32, mut termios: Termios) {
    termios.c_lflag &= !(ECHO | ICANON | ISIG);
    tcsetattr(fd, TCSAFLUSH, &termios).expect("Error updating termios constants.");
}

fn disable_raw_mode(fd: i32, original_term: Termios) {
    tcsetattr(fd, TCSAFLUSH, &original_term)
        .expect("Error resetting termios constants to default.");
}
