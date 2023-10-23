use std::sync::{Arc, Mutex};

use chess_core::{Cmd, Game, MoveCmd, Vec2};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio::{self, join};
use tui::{Areas, Event, Ui};

struct Client {
    connected: bool,
    ui: Ui,
    game: Arc<Mutex<Game>>,
}
//
impl Client {
    fn new() -> Self {
        return Client {
            connected: false,
            ui: Ui::new(),
            game: Arc::new(Mutex::new(Game::new())),
        };
    }

    async fn run(&mut self) {
        let game_ref = self.game.clone();
        tokio::spawn(async move {
            loop {
                let gamestate = { game_state_post().await.unwrap() };
                {
                    *(game_ref.lock().unwrap()) = gamestate.clone();
                }

                sleep(Duration::from_millis(30)).await;
            }
        });
        self.ui.render();
        loop {
            {
                let event = self.ui.next_event(10).await;
                Self::deal_func(&mut self.ui, event).await;

                self.ui.areas.grid_area.buffers.clear();
                {
                    for ele in &self.game.lock().unwrap().board.board {
                        self.ui
                            .areas
                            .grid_area
                            .buffers
                            .insert(ele.0.to_string(), format!("{:?}", ele.1));
                    }
                }

                self.ui.render();
            }
        }
    }

    async fn deal_func(ui: &mut Ui, event: Event) {
        match event {
            Event::ExitSignal => {
                panic!("you escaped!")
            }

            Event::TimerSignal => {}

            Event::StringInput(x) => {
                ui.areas.message.clear();
                ui.areas.message.push_str(x.as_str());
            }

            Event::GridClick(x, y) => {
                ui.areas.message.clear();

                if ui.areas.grid_area.selected == false {
                    ui.areas.grid_area.selected = true;
                    ui.areas.grid_area.select_x = x;
                    ui.areas.grid_area.select_y = y;
                } else {
                    ui.areas.grid_area.selected = false;
                    let game = game_cmd_post(
                        Vec2::new(
                            ui.areas.grid_area.select_x as i32,
                            ui.areas.grid_area.select_y as i32,
                        ),
                        Vec2::new(x as i32, y as i32),
                    )
                    .await;
                    //                    self.ui.areas.grid_area.buffers.clear();
                    //                    for ele in game.board.board {
                    //                        self.ui.areas
                    //                            .grid_area
                    //                            .buffers
                    //                            .insert(ele.0.to_string(), format!("{:?}", ele.1));
                    //                    }
                }
            }
        }
    }
}

async fn game_cmd_post(from: Vec2, to: Vec2) -> Game {
    let cmd = Cmd::Move(MoveCmd::new(from, to));

    let c = reqwest::Client::new();
    let res = c
        .post("http://localhost:8080/game/cmd")
        .json(&cmd)
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
    let game: Game = res.json().await.unwrap();
    game
    //println!("{:?}",game)
}

async fn game_state_post() -> Result<Game, &'static str> {
    let c = reqwest::Client::new();
    let res = c
        .post("http://localhost:8080/game/state")
        .send()
        .await
        .unwrap();

    let game: Game = res.json().await.unwrap();
    Ok(game)
    //println!("{:?}",game)
}

async fn example2() {
    let mut client = Client::new();
    client.run().await;
}

fn main() {
    let multi_threaded_runtime = tokio::runtime::Runtime::new().unwrap();
    multi_threaded_runtime.block_on(example2());
}
