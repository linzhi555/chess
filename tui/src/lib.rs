use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};

pub struct Ui {
    cur_pos: usize,
    buffer: Vec<char>,
    stdout: Option<RawTerminal<Stdout>>,
    message: String,
    out: String,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            cur_pos: 0,
            buffer: Vec::new(),
            stdout: None,
            message: String::new(),
            out: String::new(),
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
    fn clear_all(&mut self) {
        write!(self.stdout.as_mut().unwrap(), "{}", termion::clear::All,).unwrap();
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

    fn spawn_stdin_channel() -> Receiver<termion::event::Key> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let stdin = stdin();
            for c in stdin.keys() {
                tx.send(c.unwrap()).unwrap();
            }
        });
        rx
    }

    fn deal_new_key(&mut self, c: termion::event::Key) {
        match c {
            Key::Char('\n') => {
                self.cur_pos = 0;
                let s = self.make_string();
                self.message(&s);
                self.out = s.clone();
                self.buffer.clear();
            }
            Key::Char(c) => {
                if c.is_alphanumeric() {
                    self.insert(c)
                }
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
    }

    pub fn run(&mut self, tx: Sender<String>) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
        self.stdout = Some(stdout);

        self.message("q to exit. Type stuff, use alt, and so on.");
        self.render();

        let stdin_channel = Ui::spawn_stdin_channel();
        loop {
            let c: termion::event::Key;
            match stdin_channel.try_recv() {
                Ok(temp) => {
                    c = temp;
                    if c == Key::Ctrl('d') {
                        break;
                    }

                    self.deal_new_key(c)
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.message.push('c');
                    thread::sleep(time::Duration::from_millis(50));
                }
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }
            if !self.out.is_empty() {
                tx.send(self.out.clone()).unwrap();
                self.out.clear();
            }

            self.render();
        }
        self.clear_all();
    }
}
