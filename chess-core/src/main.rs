use lexer::{Lexer, Token};
use std::io::{self, BufRead};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct Pawn {
    pos: Vec2,
    double_start: bool,
}
impl Pawn {
    fn new(x: i32, y: i32) -> Self {
        Pawn {
            pos: Vec2 { x, y },
            double_start: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct King {
    pos: Vec2,
    castling: bool,
}
impl King {
    fn new(x: i32, y: i32) -> Self {
        King {
            pos: Vec2 { x, y },
            castling: true,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
enum Piece {
    Pawn(Pawn),
    King(King),
    None,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
enum Stage {
    WhiteTurn,
    BlackTurn,
    WhitePromotion,
    BlackPromotion,
    WhiteWin,
    BlackWin,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
struct Game {
    stage: Stage,
    board: Vec<Piece>,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            stage: Stage::WhiteTurn,
            board: Vec::new(),
        };
        game.board.push(Piece::King(King::new(1, 1)));
        game.board.push(Piece::Pawn(Pawn::new(0, 1)));
        game
    }

    fn pre_check(&mut self, c: &Cmd) -> bool {
        true
    }

    fn from_str(s: &str) -> Result<Self,()>{
        let res:serde_json::Result<Game>=serde_json::from_str(s);
        if let Ok(b) = res{
            Ok(b)
        }else{
            Err(())
        }
    }

    fn to_str(&self) -> String{
        let s = serde_json::to_string(self).unwrap();
        s
    }


    fn after_check(&mut self, c: &Cmd) -> bool {
        true
    }
}

fn get_line() -> String {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();

    line1
}

fn parse_command(s: &str) -> Result<Cmd, &str> {
    let mut lexer = Lexer::new();
    lexer.add_keyword("move");
    lexer.add_keyword("promote");
    lexer.tokenize(s);
    println!("{:?}", lexer);
    let res = parse(&lexer.result);
    res
}

fn parse(v: &Vec<Token>) -> Result<Cmd, &'static str> {
    if v.get(0).is_none() {
        return Err("cmd is empty");
    } else {
        match v.get(0).unwrap() {
            Token::Keyword(x) => {
                if x.as_str() == "move" {
                    Ok(Cmd::Move(MoveCmd {
                        from: Vec2 { x: 0, y: 1 },
                        to: Vec2 { x: 1, y: 2 },
                    }))
                } else if x.as_str() == "promote" {
                    Ok(Cmd::Promote(PromoteCmd {
                        from: Vec2 { x: 0, y: 1 },
                        to: "queen".to_string(),
                    }))
                } else {
                    Err("cmd can not parse")
                }
            }

            _ => Err("cmd can not parse"),
        }
    }
}

#[derive(Debug)]
struct MoveCmd {
    from: Vec2,
    to: Vec2,
}

#[derive(Debug)]
struct PromoteCmd {
    from: Vec2,
    to: String,
}

#[derive(Debug)]
enum Cmd {
    Move(MoveCmd),
    Promote(PromoteCmd),
}

fn main() {
    let mut game = Game::new();
    println!("{}",game.to_str());


    println!("{:?}", game);
    let mut i = -1;
    loop {
        i += 1;
        if i == 10 {
            game.stage = Stage::BlackWin;
        }

        let s = get_line();
        let c = parse_command(&s);
        if c.is_err() {
            println!("{:?}", c);
            continue;
        }
        let c = c.unwrap();
        println!("{:?}", c);

        let oldstate = game.clone();
        let res = game.pre_check(&c);
        if res != true {
            game = oldstate;
            continue;
        }

        let res = game.after_check(&c);

        if res != true {
            game = oldstate;
            continue;
        }

        println!("{:?}", game);
        match game.stage {
            Stage::WhiteWin => {
                println!("WhiteWin! the game is over");
                break;
            }
            Stage::BlackWin => {
                println!("Black Win! the game is over");
                break;
            }
            _ => {}
        }
    }
    println!("Hello, world!");
}
