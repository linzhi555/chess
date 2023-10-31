use std::sync::{Arc, Mutex};

use chess_core::{Cmd, Game, MoveCmd, Piece, PromoteCmd, Vec2};
use server;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio::{self, join};
use tui::{Areas, Event, Ui};

use lexer::{self, Token};

struct Client {
    connected: Arc<Mutex<bool>>,
    ui: Ui,
    game: Arc<Mutex<Game>>,
    id: String,
    token: String,
}
//
impl Client {
    fn new() -> Self {
        return Client {
            connected: Arc::new(Mutex::new(false)),
            ui: Ui::new(),
            game: Arc::new(Mutex::new(Game::new())),
            id: String::new(),
            token: String::new(),
        };
    }

    async fn login(&mut self) {
        let mut login_state = "id_input";

        self.ui.areas.message = "please input id".to_string();
        loop {
            let event = self.ui.next_event(10).await;

            match event {
                Event::StringInput(x) => {
                    if login_state == "id_input" {
                        self.id = x;
                        login_state = "passport_input";
                        self.ui.areas.message = "please input password".to_string();
                    } else if login_state == "passport_input" {
                        login_state = "logging";
                        self.ui.areas.message = "logging...".to_string();
                        let token = login_post(self.id.clone(), x).await;
                        if token.is_ok() {
                            self.token = token.unwrap();
                            self.ui.areas.message = "success".to_string();
                            break;
                        } else {
                            self.ui.areas.message = token.err().unwrap().to_string();
                            break;
                        }
                    }
                }

                _ => {}
            }

            self.ui.render();
        }
    }

    async fn run(&mut self) {
        let game_ref = self.game.clone();
        let connected_ref = self.connected.clone();
        tokio::spawn(async move {
            loop {
                let gamestate = { game_state_post().await };
                {
                    if gamestate.is_ok() {
                        *(game_ref.lock().unwrap()) = gamestate.unwrap().clone();
                        *connected_ref.lock().unwrap() = true;
                    } else {
                        *connected_ref.lock().unwrap() = false;
                    }
                }

                sleep(Duration::from_millis(30)).await;
            }
        });
        self.ui.render();
        loop {
            {
                let event = self.ui.next_event(10).await;
                {
                    let noconnected_msg = "can not connect to server";

                    let connected = *self.connected.lock().unwrap();
                    if !connected {
                        self.ui.areas.message = noconnected_msg.to_string();
                    }

                    Self::deal_func(&mut self.ui, event, connected).await;
                }

                self.ui.areas.grid_area.buffers.clear();
                {
                    for ele in &self.game.lock().unwrap().board.board {
                        let temp: String;
                        if ele.1.get_base().is_white() {
                            temp = Ui::color1(ele.1.get_base().name.as_str())
                        } else {
                            temp = Ui::color2(ele.1.get_base().name.as_str())
                        }

                        self.ui
                            .areas
                            .grid_area
                            .buffers
                            .insert(ele.0.to_string(), temp);
                    }
                }

                self.ui.render();
            }
        }
    }

    async fn deal_func(ui: &mut Ui, event: Event, connected: bool) {
        match event {
            Event::ExitSignal => {
                panic!("you escaped!")
            }

            Event::TimerSignal => {}

            Event::StringInput(x) => {
                ui.areas.message.clear();
                let p = parse_promot_cmd(x.as_str());
                ui.areas.message.push_str(format!("{:?}", p).as_str());
                if let Ok(piece) = p {
                    let info = game_cmd_post(Cmd::Promote(PromoteCmd {
                        from: Vec2::new(
                            ui.areas.grid_area.cur_x as i32,
                            ui.areas.grid_area.cur_y as i32,
                        ),
                        to: piece,
                    })).await;

                    ui.areas.message = info;
                }
            }

            Event::GridClick(x, y) => {
                if !connected {
                    return;
                }
                ui.areas.message.clear();

                if ui.areas.grid_area.selected == false {
                    ui.areas.grid_area.selected = true;
                    ui.areas.grid_area.select_x = x;
                    ui.areas.grid_area.select_y = y;
                } else {
                    ui.areas.grid_area.selected = false;
                    let info = game_cmd_post(Cmd::Move(MoveCmd::new(
                        Vec2::new(
                            ui.areas.grid_area.select_x as i32,
                            ui.areas.grid_area.select_y as i32,
                        ),
                        Vec2::new(x as i32, y as i32),
                    )))
                    .await;
                    ui.areas.message = info;
                }
            }
        }
    }
}

fn parse_promot_cmd(s: &str) -> Result<String, String> {
    let mut l = lexer::Lexer::new();
    l.add_keyword("promote");
    l.add_keyword("queen");
    l.add_keyword("bishop");
    l.add_keyword("rook");
    l.add_keyword("knight");
    l.tokenize(s);

    if l.result.len() == 2 {
        if let Token::Keyword(x) = l.result.get(0).unwrap().clone() {
            if x == "promote" {
                let tobe = l.result.get(1).unwrap().clone();
                match tobe {
                    Token::Keyword(x) => match x.as_str() {
                        "promotion" => return Err(String::from("can not parse a PromoteCmd")),
                        _ => return Ok(x),
                    },
                    _ => {}
                }
            }
        }
    }

    Err(String::from("can not parse a PromoteCmd"))
}

async fn game_cmd_post(cmd: Cmd) -> String {
    let c = reqwest::Client::new();
    let res = c
        .post("http://localhost:8080/game/cmd")
        .json(&cmd)
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
    let s: String = res.json().await.unwrap();
    s
    //println!("{:?}",game)
}

async fn game_state_post() -> Result<Game, &'static str> {
    let c = reqwest::Client::new();
    let res = c.post("http://localhost:8080/game/state").send().await;

    if res.is_ok() {
        let game: Game = res.unwrap().json().await.unwrap();
        Ok(game)
    } else {
        Err("server connected fail")
    }

    //println!("{:?}",game)
}

async fn login_post(id: String, password: String) -> Result<String, &'static str> {
    let c = reqwest::Client::new();
    let req = server::LoginRequest { id, password };
    let res = c
        .post("http://localhost:8080/login")
        .json(&req)
        .send()
        .await;

    if res.is_ok() {
        let response: server::LoginResponse = res.unwrap().json().await.unwrap();
        if response.ok {
            Ok(response.token)
        } else {
            Err("login fail")
        }
    } else {
        Err("server connected fail")
    }

    //println!("{:?}",game)
}

async fn example2() {
    let mut client = Client::new();
    //client.login().await;
    client.run().await;
}

fn main() {
    let multi_threaded_runtime = tokio::runtime::Runtime::new().unwrap();
    multi_threaded_runtime.block_on(example2());
}
