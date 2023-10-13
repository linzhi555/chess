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
pub struct Pawn {
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
pub struct King {
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
pub enum Piece {
    Pawn(Pawn),
    King(King),
}



#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Stage {
    WhiteTurn,
    BlackTurn,
    WhitePromotion,
    BlackPromotion,
    WhiteWin,
    BlackWin,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Game {
    stage: Stage,
    board: Vec<Piece>,
}

impl Game {
    pub fn new() -> Self {
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

    pub fn get_piece(&self,x:u32,y:u32)->Option<String>{
        for p in self.board.iter(){
            let (pos,name) =  match p {
                Piece::Pawn(x) =>  {
                    (x.pos.clone(),"Pawn".to_string())
                    
                }
                Piece::King(x) =>{
                    (x.pos.clone(),"King".to_string())
                }
            };

            if pos.x == x as i32  && pos.y== y as i32 {
                return Some(name)
            }
        }
        return None
    }

    pub fn from_str(s: &str) -> Result<Self,()>{
        let res:serde_json::Result<Game>=serde_json::from_str(s);
        if let Ok(b) = res{
            Ok(b)
        }else{
            Err(())
        }
    }

    pub fn to_str(&self) -> String{
        let s = serde_json::to_string(self).unwrap();
        s
    }


    fn after_check(&mut self, c: &Cmd) -> bool {
        true
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
