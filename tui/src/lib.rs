use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc::Sender;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};

pub struct Ui {
    cur_pos: usize,
    buffer: Vec<char>,
    stdout: Option<RawTerminal<Stdout>>,
    message: String,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            cur_pos: 0,
            buffer: Vec::new(),
            stdout: None,
            message: String::new(),
        }
    }

    fn make_string(&self) -> String {
        let mut s = String::new();
        for c in self.buffer.iter() {
            s.push(*c)
        }
        s
    }

    fn move_cursor(&mut self, i: u16) {
        write!(
            self.stdout.as_mut().unwrap(),
            "{}",
            termion::cursor::Goto(i, 1)
        )
        .unwrap();
    }

    fn insert(&mut self, c: char) {
        self.buffer.insert(self.cur_pos, c);
        self.cur_pos += 1;
    }
    fn delete(&mut self) {
        if self.cur_pos >= 1 {
            self.cur_pos -= 1;
            self.buffer.remove(self.cur_pos);
        }
    }

    fn message(&mut self, s: &str) {
        self.message = s.to_string()
    }
    fn clear_all(&mut self){
        write!(self.stdout.as_mut().unwrap(),"{}", termion::clear::All,).unwrap();
        self.stdout.as_mut().unwrap().flush().unwrap();
    }

    fn render(&mut self) {
        write!(
            self.stdout.as_mut().unwrap(),
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 3),
            self.message,
        )
        .unwrap();

        let mut i = 0;
        let mut cursor = 0;
        self.move_cursor(1);
        write!(
            self.stdout.as_mut().unwrap(),
            "{}> ",
            termion::clear::CurrentLine
        )
        .unwrap();

        for c in self.buffer.iter() {
            print!("{}", c);
            if i < self.cur_pos {
                cursor += 1;
                if !c.is_ascii() {
                    cursor += 1
                }
            }
            i += 1;
        }
        self.move_cursor(cursor + 3);
        self.stdout.as_mut().unwrap().flush().unwrap();
    }

    pub fn run(&mut self,tx:Sender<String>) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
        self.stdout = Some(stdout);

        self.message("q to exit. Type stuff, use alt, and so on.");
        self.render();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    self.cur_pos = 0;
                    let s = self.make_string();
                    self.message(&s);
                    self.buffer.clear();
                    tx.send(s.clone()).unwrap();
                }
                Key::Char(c) => {
                    if c.is_alphanumeric() {
                        self.insert(c)
                    }
                }
                Key::Ctrl('d') => {
                    break;
                }
                Key::Left => {
                    if self.cur_pos >= 1 {
                        self.cur_pos -= 1
                    }
                }
                Key::Right => {
                    if self.cur_pos <= self.buffer.len() {
                        self.cur_pos += 1
                    }
                }
                Key::End => self.cur_pos = self.buffer.len(),
                Key::Home => self.cur_pos = 0,
                //Key::Up => print!("↑"),
                //Key::Down => print!("↓"),
                Key::Backspace => self.delete(),
                _ => {}
            }
            self.render();
        }
        self.clear_all();
    }
}
