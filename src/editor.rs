use crate::Terminal;
use std::io::stdout;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error)
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = self.terminal.read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialise terminal."),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        self.terminal.clear_screen();
        self.terminal.cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r")
        } else {
            self.draw_rows();
            self.terminal.cursor_position(0, 0);
        }
        self.terminal.flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!(e);
}
