use std::io;

use termion::event::Key;

use crate::Terminal;

pub struct Editor {
    terminal: Terminal,
    running: bool,
}

impl Editor {
    pub fn new() -> Self {
        let terminal = Terminal::default().expect("Failed to initialize terminal");

        Self {
            terminal,
            running: true,
        }
    }

    fn handle_key_event(&mut self) {
        match Terminal::read_key() {
            Ok(key) => match key {
                Key::Char('q') => {
                    self.running = false;
                }
                Key::Char(c) => {
                    Terminal::clear_screen();
                    print!("{}", c);
                }
                _ => (),
            },
            Err(error) => {
                die(&error);
            }
        }
    }

    fn draw(&self) -> Result<(), io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn start(&mut self) {
        Terminal::clear_screen();
        Terminal::cursor_save();

        while self.running {
            if let Err(error) = self.draw() {
                die(&error);
            }

            self.handle_key_event();
        }

        Terminal::cursor_restore();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
    }
}

fn die(e: &std::io::Error) {
    Terminal::cursor_restore();
    Terminal::clear_screen();
    panic!("{:?}", e);
}
