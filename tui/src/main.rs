use std::thread;
use std::time::Duration;
use tui::{Ui,Frame};
use lexer;
use lexer::{Lexer,Token};
use chess_core::Game;
fn main() {
    let (mut ui, tx, rx) = Ui::new();
    let handle = thread::spawn(move || {
        ui.run();
    });
    let game = Game::new();
    let mut buffer = String::new();
    while !handle.is_finished() {
        if let Ok(temp) = rx.recv_timeout(Duration::from_millis(100)) {
            let mut reply = "the parse res".to_string();
            let tokens = lexer::Lexer::to_token_vec(temp.as_str());
            let tokens_str = format!("{:?}",tokens);
            reply.push_str(tokens_str.as_str());
            reply.push_str(game.to_str().as_str());
            //tx.send(reply).unwrap();
            tx.send(game.to_str()).unwrap();
            //tx.send(draw_board()).unwrap();
            buffer.push_str(temp.as_str())
        }
    }
    println!("sub process exited the result is {}", buffer)
}

fn draw_board() -> Frame{
    let mut  lines = Vec::new();
    lines.push("------------------------".to_string());
    lines.push("|RO|KN|BI|QU|KI|BI|KN|RO|".to_string());
    lines.push("------------------------".to_string());
    lines.push("|PA|PA|PA|PA|PA|PA|PA|PA|".to_string());
    lines.push("------------------------".to_string());
    lines.push("|  |  |  |  |  |  |  |  |".to_string());
    lines.push("------------------------".to_string());
    lines.push("|  |  |  |  |  |  |  |  |".to_string());
    lines.push("------------------------".to_string());
    lines.push("|  |  |  |  |  |  |  |  |".to_string());
    lines.push("------------------------".to_string());
    lines.push("|  |  |  |  |  |  |  |  |".to_string());
    lines.push("------------------------".to_string());
    lines.push("|pa|pa|pa|pa|pa|pa|pa|pa|".to_string());
    lines.push("------------------------".to_string());
    lines.push("|ro|kn|bi|qu|ki|bi|kn|ro|".to_string());
    lines.push("------------------------".to_string());
    Frame::from_vec(lines)

}
