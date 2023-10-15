use core::time;
use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};

pub struct Frame {
    lines: Vec<String>,
}
impl Frame {
    pub fn one_line_frame(s: &str) -> Self {
        let mut lines = Vec::new();
        lines.push(s.to_string());
        Frame { lines }
    }

    pub fn from_vec(lines: Vec<String>) -> Self {
        Frame { lines }
    }
}

struct GridArea {
    cur_x: u32,
    cur_y: u32,
}
impl GridArea {
    fn deal_new_key(&mut self, c: termion::event::Key) -> String {
        let mut res = String::new();
        match c {
            Key::Char('\n') => {
                res = format!("{} {}", self.cur_x, self.cur_y);
            }

            Key::Left => {
                if self.cur_x > 0 {
                    self.cur_x -= 1
                }
            }
            Key::Right => {
                if self.cur_x < 7 {
                    self.cur_x += 1
                }
            }
            Key::Up => {
                if self.cur_y < 7 {
                    self.cur_y += 1
                }
            }
            Key::Down => {
                if self.cur_y > 0 {
                    self.cur_y -= 1
                }
            }

            _ => {}
        }
        res
    }

    fn render(&self) -> Frame {
        let mut lines = Vec::new();

        for y in (0..8).rev() {
            lines.push("--------------------------------".to_string());
            let mut temp = String::new();
            for x in 0..8 {
                temp.push_str("|");
                if x == self.cur_x && y == self.cur_y {
                    temp.push_str("-> ");
                } else {
                    temp.push_str("   ");
                }
            }

            temp.push_str("|");
            lines.push(temp);
        }
        return Frame::from_vec(lines);
    }
}

struct InputArea {
    cur_pos: usize,
    buffer: Vec<char>,
}
impl InputArea {
    fn make_string(&self) -> String {
        let mut s = String::new();
        for c in self.buffer.iter() {
            s.push(*c)
        }
        s
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

    fn render(&mut self) -> Frame {
        let mut lines = Vec::new();
        let mut temp = String::new();
        temp.push_str("> ");
        for c in self.buffer.iter() {
            temp.push(*c)
        }
        lines.push(temp);
        return Frame::from_vec(lines);
    }

    fn deal_new_key(&mut self, c: termion::event::Key) -> String {
        let mut res = String::new();
        match c {
            Key::Char('\n') => {
                self.cur_pos = 0;
                let s = self.make_string();
                res = s;
                self.buffer.clear();
            }
            Key::Char(c) => {
                if !c.is_control() {
                    self.insert(c)
                }
            }
            Key::Left => {
                if self.cur_pos >= 1 {
                    self.cur_pos -= 1
                }
            }
            Key::Right => {
                if self.cur_pos < self.buffer.len() {
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
        res
    }
}

enum UiFocus {
    GridArea,
    InputArea,
}

impl UiFocus {
    fn switch(&mut self) {
        match self {
            UiFocus::InputArea => *self = UiFocus::GridArea,
            UiFocus::GridArea => *self = UiFocus::InputArea,
        }
    }
}

pub struct Ui {
    focus: UiFocus,
    grid_area: GridArea,
    input_area: InputArea,
    event_handle: Box<dyn FnMut(&str, &mut String)>,
    message: String,
    stdout: Option<RawTerminal<Stdout>>,
}

fn default_handle(s: &str, message: &mut String) {
    message.clear();
    message.push_str("dafault handle");
    message.push_str(s);
}

impl Ui {
    pub fn new(deal_func: Box<dyn FnMut(&str, &mut String)>) -> Self {
        Ui {
            focus: UiFocus::InputArea,
            grid_area: GridArea { cur_x: 0, cur_y: 0 },
            input_area: InputArea {
                cur_pos: 0,
                buffer: Vec::new(),
            },
            event_handle: deal_func,
            message: String::new(),
            stdout: None,
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
        write!(self.stdout.as_mut().unwrap(), "{}", termion::clear::All,).unwrap();

        let mut i = 0;
        for l in self.grid_area.render().lines.iter() {
            write!(
                self.stdout.as_mut().unwrap(),
                "{}{}",
                termion::cursor::Goto(1, 6 + i),
                l,
            )
            .unwrap();
            i += 1;
        }

        let mut i = 0;
        for l in self.input_area.render().lines.iter() {
            write!(
                self.stdout.as_mut().unwrap(),
                "{}{}",
                termion::cursor::Goto(1, 3 + i),
                l,
            )
            .unwrap();
            i += 1;
        }

        match self.focus {
            UiFocus::InputArea => {
                write!(
                    self.stdout.as_mut().unwrap(),
                    "{}===>",
                    termion::cursor::Goto(1, 1),
                )
                .unwrap();
            }
            UiFocus::GridArea => write!(
                self.stdout.as_mut().unwrap(),
                "{}===>",
                termion::cursor::Goto(1, 5),
            )
            .unwrap(),
        }

        write!(
            self.stdout.as_mut().unwrap(),
            "{}{}",
            termion::cursor::Goto(1, 30),
            self.message,
        )
        .unwrap();

        write!(
            self.stdout.as_mut().unwrap(),
            "{}",
            termion::cursor::Goto(self.input_area.cur_pos as u16 + 3, 3),
        )
        .unwrap();

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

    pub fn run(&mut self) {
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

                    if c == Key::Char('\t') {
                        self.focus.switch();
                        continue;
                    }

                    match self.focus {
                        UiFocus::InputArea => {
                            let m = self.input_area.deal_new_key(c);
                            (*self.event_handle)(m.as_str(), &mut self.message)
                        }
                        UiFocus::GridArea => {
                            let m = self.grid_area.deal_new_key(c);
                            (*self.event_handle)(m.as_str(), &mut self.message)
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            self.render();
            thread::sleep(time::Duration::from_millis(1))
        }
        self.clear_all();
    }
}
