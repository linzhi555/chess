use std::sync::{Arc,Mutex};

use chess_core::{Cmd, Game, MoveCmd, Vec2};
use tokio::{self, join};
use tui::{Areas, Event, Ui};
use tokio::sync::mpsc;
use tokio::time::{sleep,Duration};

struct Client {
    connected: bool,
    ui:Ui
}
//
impl Client {

    fn new() -> Self{
        return Client { connected: false, ui: Ui::new() }
    }

    async fn run(&mut self) {

        let (tx,mut rx)= mpsc::channel(1);
        tokio::spawn(async move
            {
                loop{

                    let game = game_state_post().await;
                    tx.send(game.unwrap()).await.unwrap();
                }

            }
        );

        self.ui.render();
        loop {
            let (event,gamestate) = join!(self.ui.next_event(10), rx.recv());
            self.deal_func(event).await;
            
            {
                self.ui.areas.grid_area.buffers.clear();
                for ele in gamestate.unwrap().board.board {
                    self.ui.areas
                        .grid_area
                        .buffers
                        .insert(ele.0.to_string(), format!("{:?}", ele.1));
                }
            }

            self.ui.render()
        }
    }


    async fn deal_func(&mut self,event: Event)  {
        match event {
            Event::ExitSignal => {
                panic!("you escaped!")
            }
            
            Event::TimerSignal => {
            }

            Event::StringInput(x) => {
                self.ui.areas.message.clear();
                self.ui.areas.message.push_str(x.as_str());
            }

            Event::GridClick(x, y) => {

                self.ui.areas.message.clear();

                if self.ui.areas.grid_area.selected == false {
                    self.ui.areas.grid_area.selected = true;
                    self.ui.areas.grid_area.select_x = x;
                    self.ui.areas.grid_area.select_y = y;
                } else {
                    self.ui.areas.grid_area.selected = false;
                    let game = game_cmd_post(
                        Vec2::new(
                            self.ui.areas.grid_area.select_x as i32,
                            self.ui.areas.grid_area.select_y as i32,
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
    let mut  client = Client::new();
    client.run().await;
}

fn main() {
    let multi_threaded_runtime = tokio::runtime::Runtime::new().unwrap();
    multi_threaded_runtime.block_on(example2());
}
