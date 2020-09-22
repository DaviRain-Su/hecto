
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self{
            should_quit: false,
        }
    }
    pub fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let press_key = read_key()?;
        match press_key {
            Key::Ctrl('q') => {
                self.should_quit = true;
            },
            Key::Char(c) => {
                if c.is_control() {
                    println!("{}\r", c as u8);
                }else {
                    println!("{} ({})\r", c as u8, c)
                }
            },
            _ => (),
        }
        Ok(())
    }

    pub fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}", termion::clear::All);
        io::stdout().flush()
    }
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}