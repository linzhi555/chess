use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};

pub struct Ui {
    bufferWidth: isize,
    buffer: Vec<char>,
    stdout: Option<RawTerminal<Stdout>>,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            bufferWidth: 0,
            buffer: Vec::new(),
            stdout: None,
        }
    }

    fn move_cursor(&mut self, i: u16) {
        write!(
            self.stdout.as_mut().unwrap(),
            "{}",
            termion::cursor::Goto(i, 2)
        )
        .unwrap();
    }

    pub fn run(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        write!(
            stdout,
            "{}{}q to exit. Type stuff, use alt, and so on.",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
        stdout.flush().unwrap();
        self.stdout = Some(stdout);
        let mut i = 1;
        for c in stdin.keys() {
            self.stdout.as_mut().unwrap().flush().unwrap();
            match c.unwrap() {
                Key::Char(c) => {
                    print!("{}", c);
                    if !c.is_ascii() {
                        i += 2;
                        self.move_cursor(i);
                    } else {
                        i += 1;
                        self.move_cursor(i);
                    }
                }
                Key::Alt(c) => {
                    print!("^{}", c);
                    i += 2;
                    self.move_cursor(i);
                }
                Key::Ctrl('d') => {
                    break;
                }
                //Key::Esc => print!("ESC"),
                Key::Left => {
                    i -= 1;
                    self.move_cursor(i);
                }
                Key::Right => {
                    i += 1;
                    self.move_cursor(i);
                }
                //Key::Up => print!("↑"),
                //Key::Down => print!("↓"),
                Key::Backspace => {
                    print!(" ");
                    i += 1;
                    self.move_cursor(i);
                }
                _ => {}
            }

            self.stdout.as_mut().unwrap().flush().unwrap();
        }
    }
}
