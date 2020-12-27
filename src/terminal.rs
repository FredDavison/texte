pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(&self, x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x + 1, y + 1));
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
