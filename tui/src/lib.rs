use core::time;
use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
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

pub struct Ui {
    cur_pos: usize,
    buffer: Vec<char>,
    stdout: Option<RawTerminal<Stdout>>,
    message: String,
    rx_input: Receiver<Frame>,
    tx_output: Sender<String>,
    frame: Frame,
    grid_area: GridArea,
    out: String,
}

impl Ui {
    pub fn new() -> (Self, Sender<Frame>, Receiver<String>) {
        let (tx_input, rx_input) = mpsc::channel();
        let (tx_output, rx_output) = mpsc::channel();
        (
            Ui {
                cur_pos: 0,
                buffer: Vec::new(),
                stdout: None,
                message: String::new(),
                out: String::new(),
                rx_input,
                tx_output,
                grid_area: GridArea { cur_x: 0, cur_y: 0 },
                frame: Frame::one_line_frame("frame"),
            },
            tx_input,
            rx_output,
        )
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
        for l in self.frame.lines.iter() {
            write!(
                self.stdout.as_mut().unwrap(),
                "{}{}",
                termion::cursor::Goto(1, 5 + i),
                l,
            )
            .unwrap();
            i += 1;
        }

        i = 0;
        for l in self.grid_area.render().lines.iter() {
            write!(
                self.stdout.as_mut().unwrap(),
                "{}{}",
                termion::cursor::Goto(1, 15 + i),
                l,
            )
            .unwrap();
            i += 1;
        }

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
                if self.grid_area.cur_x > 0 {
                    self.grid_area.cur_x -= 1
                }
                if self.cur_pos >= 1 {
                    self.cur_pos -= 1
                }
            }
            Key::Right => {
                if self.grid_area.cur_x < 7 {
                    self.grid_area.cur_x += 1
                }
                if self.cur_pos <= self.buffer.len() {
                    self.cur_pos += 1
                }
            }
            Key::Up => {
                if self.grid_area.cur_y < 7 {
                    self.grid_area.cur_y += 1
                }
            }
            Key::Down => {
                if self.grid_area.cur_y > 0 {
                    self.grid_area.cur_y -= 1
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

                    self.deal_new_key(c)
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            match self.rx_input.try_recv() {
                Ok(temp) => self.frame = temp,
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            if !self.out.is_empty() {
                self.tx_output.send(self.out.clone()).unwrap();
                self.out.clear();
            }

            self.render();
            thread::sleep(time::Duration::from_millis(1))
        }
        self.clear_all();
    }
}
