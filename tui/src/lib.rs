use core::time;
use std::future::Future;
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

pub enum Event {
    StringInput(String),
    GridClick(u32, u32),
}

#[derive(Clone)]
pub struct GridArea {
    cur_x: u32,
    cur_y: u32,
}
impl GridArea {
    fn deal_new_key(&mut self, c: termion::event::Key) -> Option<Event> {
        let mut res: Option<Event> = None;
        match c {
            Key::Char('\n') => {
                res = Some(Event::GridClick(self.cur_x, self.cur_y));
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

#[derive(Clone)]
pub struct InputArea {
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

    fn deal_new_key(&mut self, c: termion::event::Key) -> Option<Event> {
        let mut res = None;
        match c {
            Key::Char('\n') => {
                self.cur_pos = 0;
                let s = self.make_string();
                res = Some(Event::StringInput(s));
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
#[derive(Clone)]
pub struct Areas {
    pub input_area: InputArea,
    pub grid_area: GridArea,
    pub message: String,
}

pub struct Ui<F>
where
    F: std::future::Future<Output = Areas>,
{
    focus: UiFocus,
    areas: Areas,
    event_handle: fn(event: Event, areas: Areas) -> F,
    stdout: Option<RawTerminal<Stdout>>,
}

impl<F> Ui<F>
where
    F: std::future::Future<Output = Areas>,
{
    pub fn new(foo: fn(event: Event, areas: Areas) -> F) -> Self {
        Ui {
            focus: UiFocus::InputArea,
            areas: Areas {
                grid_area: GridArea { cur_x: 0, cur_y: 0 },
                input_area: InputArea {
                    cur_pos: 0,
                    buffer: Vec::new(),
                },

                message: String::new(),
            },
            event_handle: foo,
            stdout: None,
        }
    }

    fn message(&mut self, s: &str) {
        self.areas.message = s.to_string()
    }
    fn clear_all(&mut self) {
        write!(self.stdout.as_mut().unwrap(), "{}", termion::clear::All,).unwrap();
        self.stdout.as_mut().unwrap().flush().unwrap();
    }

    fn render(&mut self) {
        write!(self.stdout.as_mut().unwrap(), "{}", termion::clear::All,).unwrap();

        let mut i = 0;
        for l in self.areas.grid_area.render().lines.iter() {
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
        for l in self.areas.input_area.render().lines.iter() {
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
            self.areas.message,
        )
        .unwrap();

        write!(
            self.stdout.as_mut().unwrap(),
            "{}",
            termion::cursor::Goto(self.areas.input_area.cur_pos as u16 + 3, 3),
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

    pub async fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        stdout.flush().unwrap();
        self.stdout = Some(stdout);

        self.message("q to exit. Type stuff, use alt, and so on.");
        self.render();

        let stdin_channel = Self::spawn_stdin_channel();
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

                    let event: Option<Event>;

                    match self.focus {
                        UiFocus::InputArea => {
                            event = self.areas.input_area.deal_new_key(c);
                        }
                        UiFocus::GridArea => {
                            event = self.areas.grid_area.deal_new_key(c);
                        }
                    };

                    if event.is_some() {
                        self.areas = (self.event_handle)(event.unwrap(), self.areas.clone()).await;
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
