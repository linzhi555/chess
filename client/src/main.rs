use chess_core::{Cmd, Game, MoveCmd, Vec2};
use tokio;
use tui::{Areas, Event, Ui};

async fn deal_func(event: Event, areas: Areas) -> Areas {
    let mut areas = areas;

    match event {
        Event::TimerSignal => {
            let game = game_state_post().await;
            areas.grid_area.buffers.clear();
            for ele in game.board.board {
                areas
                    .grid_area
                    .buffers
                    .insert(ele.0.to_string(), format!("{:?}", ele.1));
            }
        }

        Event::StringInput(x) => {
            areas.message.clear();
            areas.message.push_str(x.as_str());
        }

        Event::GridClick(x, y) => {
            areas.message.clear();

            if areas.grid_area.selected == false {
                areas.grid_area.selected = true;
                areas.grid_area.select_x = x;
                areas.grid_area.select_y = y;
            } else {
                areas.grid_area.selected = false;
                let game = game_cmd_post(
                    Vec2::new(
                        areas.grid_area.select_x as i32,
                        areas.grid_area.select_y as i32,
                    ),
                    Vec2::new(x as i32, y as i32),
                )
                .await;
                areas.grid_area.buffers.clear();
                for ele in game.board.board {
                    areas
                        .grid_area
                        .buffers
                        .insert(ele.0.to_string(), format!("{:?}", ele.1));
                }
            }
        }
    }

    areas
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

async fn game_state_post() -> Game {
    let c = reqwest::Client::new();
    let res = c
        .post("http://localhost:8080/game/state")
        .send()
        .await
        .unwrap();

    let game: Game = res.json().await.unwrap();
    game
    //println!("{:?}",game)
}

async fn example2() {
    let mut ui = Ui::new(deal_func);
    ui.run().await;
}

fn main() {
    let multi_threaded_runtime = tokio::runtime::Runtime::new().unwrap();
    multi_threaded_runtime.block_on(example2());
}
