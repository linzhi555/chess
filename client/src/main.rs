use chess_core::{Cmd, Game, MoveCmd, Vec2};
use tui;
#[tokio::main]
async fn main() {
    post().await
}

async fn post() {
    let cmd = Cmd::Move(MoveCmd::new(Vec2::new(3, 1), Vec2::new(3, 7)));

    let c = reqwest::Client::new();
    let res = c
        .post("http://localhost:8080/game")
        .json(&cmd)
        .send()
        .await
        .unwrap();

    println!("{:?}", res);
    println!("{:?}", res.bytes().await);
    //let game:Game = res.json().await.unwrap();

    //println!("{:?}",game)
}
