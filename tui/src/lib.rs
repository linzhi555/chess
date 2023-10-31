use core::time;
use std::collections::HashMap;
use std::future::Future;
use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::color;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};
use tokio::time::{sleep, Duration};
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
    TimerSignal,
    ExitSignal,
}

#[derive(Clone)]
pub struct GridArea {
    pub cur_x: u32,
    pub cur_y: u32,

    pub selected: bool,
    pub select_x: u32,
    pub select_y: u32,
    pub buffers: HashMap<String, String>,
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
        let empty = "    ";
        let arrow = Ui::color2(" -> ");
        let selec = " v  ";
        for y in (0..8).rev() {
            lines.push("--------------------------------------".to_string());
            let mut temp = String::new();
            for x in 0..8 {
                temp.push_str("|");

                if self.selected && x == self.select_x && y == self.select_y {
                    temp.push_str(selec);
                } else if x == self.cur_x && y == self.cur_y {
                    temp.push_str(arrow.as_str());
                } else {
                    temp.push_str(empty);
                }
            }

            temp.push_str("|");
            lines.push(temp);

            let mut temp = String::new();
            for x in 0..8 {
                let key = format!("({},{})", x, y);
                let val = self.buffers.get(key.as_str());
                temp.push_str("|");

                if val.is_some() {
                    let s = val.unwrap().clone();
                    temp.push_str(s.as_str());
                } else {
                    temp.push_str(empty);
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

pub struct Ui {
    focus: UiFocus,
    pub areas: Areas,
    stdout: RawTerminal<Stdout>,
    stdin_channel: Receiver<termion::event::Key>,
    pub time_counter: usize,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            focus: UiFocus::InputArea,
            areas: Areas {
                grid_area: GridArea {
                    cur_x: 0,
                    cur_y: 0,
                    buffers: HashMap::new(),
                    select_x: 0,
                    select_y: 0,
                    selected: false,
                },
                input_area: InputArea {
                    cur_pos: 0,
                    buffer: Vec::new(),
                },

                message: String::new(),
            },
            stdout: stdout().into_raw_mode().unwrap(),
            stdin_channel: Self::spawn_stdin_channel(),
            time_counter: 0,
        }
    }

    fn message(&mut self, s: &str) {
        self.areas.message = s.to_string()
    }
    fn clear_all(&mut self) {
        write!(self.stdout, "{}", termion::clear::All,).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn render(&mut self) {
        write!(self.stdout, "{}", termion::clear::All,).unwrap();

        let mut i = 0;
        for l in self.areas.grid_area.render().lines.iter() {
            write!(self.stdout, "{}{}", termion::cursor::Goto(1, 6 + i), l,).unwrap();
            i += 1;
        }

        let mut i = 0;
        for l in self.areas.input_area.render().lines.iter() {
            write!(self.stdout, "{}{}", termion::cursor::Goto(1, 3 + i), l,).unwrap();
            i += 1;
        }

        match self.focus {
            UiFocus::InputArea => {
                write!(self.stdout, "{}===>", termion::cursor::Goto(1, 1),).unwrap();
            }
            UiFocus::GridArea => {
                write!(self.stdout, "{}===>", termion::cursor::Goto(1, 5),).unwrap()
            }
        }

        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 30),
            self.areas.message,
        )
        .unwrap();

        write!(
            self.stdout,
            "{}",
            termion::cursor::Goto(self.areas.input_area.cur_pos as u16 + 3, 3),
        )
        .unwrap();

        self.stdout.flush().unwrap();
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

    pub async fn next_event(&mut self, timeout: usize) -> Event {
        for _i in 0..timeout {
            let c: termion::event::Key;
            match self.stdin_channel.try_recv() {
                Ok(temp) => {
                    c = temp;
                    if c == Key::Ctrl('d') {
                        return Event::ExitSignal;
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
                        return event.unwrap();
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }
            sleep(Duration::from_millis(1)).await;
        }
        return Event::TimerSignal;
    }
    pub fn base_color(s: &str) -> String {
        format!(
            "{}{}{}",
            color::Fg(color::White),
            s,
            color::Fg(color::White)
        )
    }

    pub fn color1(s: &str) -> String {
        format!(
            "{}{}{}",
            color::Fg(color::Green),
            s,
            color::Fg(color::White)
        )
    }

    pub fn color2(s: &str) -> String {
        format!("{}{}{}", color::Fg(color::Red), s, color::Fg(color::White))
    }

    //pub async fn run(&mut self) {
    //    let mut stdout = stdout().into_raw_mode().unwrap();
    //    stdout.flush().unwrap();
    //    self.stdout = Some(stdout);

    //    self.message("q to exit. Type stuff, use alt, and so on.");
    //    self.render();

    //    let stdin_channel = Self::spawn_stdin_channel();
    //    loop {
    //        let c: termion::event::Key;
    //        match stdin_channel.try_recv() {
    //            Ok(temp) => {
    //                c = temp;
    //                if c == Key::Ctrl('d') {
    //                    break;
    //                }

    //                if c == Key::Char('\t') {
    //                    self.focus.switch();
    //                    continue;
    //                }

    //                let event: Option<Event>;

    //                match self.focus {
    //                    UiFocus::InputArea => {
    //                        event = self.areas.input_area.deal_new_key(c);
    //                    }
    //                    UiFocus::GridArea => {
    //                        event = self.areas.grid_area.deal_new_key(c);
    //                    }
    //                };

    //                if event.is_some() {
    //                    self.areas = (self.event_handle)(event.unwrap(), self.areas.clone()).await;
    //                }
    //            }
    //            Err(mpsc::TryRecvError::Empty) => {}
    //            Err(mpsc::TryRecvError::Disconnected) => panic!("Channel disconnected"),
    //        }

    //        if self.time_counter % 20 == 0 {
    //            self.areas = (self.event_handle)(Event::TimerSignal, self.areas.clone()).await;
    //        }
    //        self.time_counter += 1;

    //        self.render();
    //        thread::sleep(time::Duration::from_millis(1))
    //    }
    //    self.clear_all();
    //}
}
