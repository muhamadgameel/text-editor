use std::io::{self, stdin, stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Terminal {
    pub size: (usize, usize),
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    /// # Errors
    ///
    /// Will return `Err` if termion failed to get `terminal_size`
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;

        Ok(Self {
            size: (size.0 as usize, size.1.saturating_sub(2) as usize),
            _stdout: stdout().into_raw_mode()?,
        })
    }

    /// # Errors
    ///
    /// Will return `Err` if failed to read terminal input key
    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if failed to flush stdout buffer
    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn cursor_save() {
        print!("{}", termion::cursor::Save);
    }

    pub fn cursor_restore() {
        print!("{}", termion::cursor::Restore);
    }

    pub fn cursor_position(x: u16, y: u16) {
        print!(
            "{}",
            termion::cursor::Goto(x.saturating_add(1), y.saturating_add(1))
        );
    }
}
