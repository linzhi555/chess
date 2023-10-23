use lexer;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Serialize, Deserialize, Copy, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }

    pub fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }

    pub fn from_str(s: &str) -> Result<Self, ()> {
        let token_vec = lexer::Lexer::to_token_vec(s);
        if token_vec.len() != 5 {
            return Err(());
        }

        let mut res = Vec2::new(0, 0);

        if *(token_vec.get(0).unwrap()) != lexer::Token::LParentheses {
            return Err(());
        }

        if let lexer::Token::Int(x) = token_vec.get(1).unwrap() {
            res.x = *x;
        } else {
            return Err(());
        }

        if *(token_vec.get(2).unwrap()) != lexer::Token::Comma {
            return Err(());
        }

        if let lexer::Token::Int(y) = token_vec.get(3).unwrap() {
            res.y = *y;
        } else {
            return Err(());
        }
        if *(token_vec.get(4).unwrap()) != lexer::Token::RParentheses {
            return Err(());
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2;

    #[test]
    fn vec2_conver() {
        if Vec2::new(1, 2) != Vec2::from_str("(1,2)").unwrap() {
            panic!("Err when vec2 convert")
        }

        if Vec2::new(4, 2) != Vec2::from_str("( 4 ,2 )").unwrap() {
            panic!("Err when vec2 convert")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub enum Camp {
    White,
    Black,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasePiece {
    pub pos: Vec2,
    pub camp: Camp,
    pub name: String,
}

impl BasePiece {
    fn new(x: i32, y: i32, camp: Camp, name: String) -> Self {
        BasePiece {
            pos: Vec2 { x, y },
            camp,
            name,
        }
    }
    pub fn is_white(&self) -> bool {
        if self.camp == Camp::White {
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pawn {
    base: BasePiece,
    double_start: bool,
}

impl Pawn {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Pawn {
            double_start: false,
            base: BasePiece::new(x, y, camp, "pawn".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct King {
    base: BasePiece,
    castling: bool,
}
impl King {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        King {
            castling: true,
            base: BasePiece::new(x, y, camp, "king".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Piece {
    Pawn(Pawn),
    King(King),
}

impl Piece {
    pub fn get_base(&self) -> BasePiece {
        match self {
            Piece::Pawn(p) => p.base.clone(),
            Piece::King(p) => p.base.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Stage {
    WhiteTurn,
    BlackTurn,
    WhitePromotion,
    BlackPromotion,
    WhiteWin,
    BlackWin,
}

const ERR_PIECE_NOT_FOUND: &'static str = "piece not found";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChessBoard {
    pub board: HashMap<String, Piece>,
}

impl ChessBoard {
    fn new() -> Self {
        return ChessBoard {
            board: HashMap::new(),
        };
    }

    fn insert_piece(&mut self, p: Piece) {
        self.board.insert(p.get_base().pos.to_string(), p);
    }

    fn move_piece(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        let p: Piece;
        if let Some(res) = self.board.get(from.to_string().as_str()) {
            p = res.clone();
        } else {
            return Err(ERR_PIECE_NOT_FOUND);
        }

        self.board.remove(&from.to_string()).unwrap();
        self.board.insert(to.to_string(), p);

        Ok(())
    }

    fn get_piece_mut(&mut self, pos: Vec2) -> Option<&mut Piece> {
        self.board.get_mut(pos.to_string().as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    stage: Stage,
    pub board: ChessBoard,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            stage: Stage::WhiteTurn,
            board: ChessBoard::new(),
        };

        game.board
            .insert_piece(Piece::King(King::new(3, 0, Camp::White)));
        game.board
            .insert_piece(Piece::Pawn(Pawn::new(3, 1, Camp::White)));

        game.board
            .insert_piece(Piece::King(King::new(3, 7, Camp::Black)));
        game.board
            .insert_piece(Piece::Pawn(Pawn::new(3, 6, Camp::Black)));

        game
    }

    pub fn exec_cmd(&mut self, c: &Cmd) -> Result<(), &'static str> {
        match c {
            Cmd::Move(x) => self.deal_move(x.from, x.to),

            Cmd::Promote(_) => Ok(()),
        }
    }

    fn deal_move(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        if let Some(p) = self.board.get_piece_mut(from) {
            self.board.move_piece(from, to)
        } else {
            Err("piece can not find")
        }
    }

    fn pre_check(&mut self, c: &Cmd) -> bool {
        true
    }

    pub fn from_str(s: &str) -> Result<Self, ()> {
        let res: serde_json::Result<Game> = serde_json::from_str(s);
        if let Ok(b) = res {
            Ok(b)
        } else {
            Err(())
        }
    }

    pub fn to_str(&self) -> String {
        let s = serde_json::to_string(self).unwrap();
        s
    }

    fn after_check(&mut self, c: &Cmd) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveCmd {
    from: Vec2,
    to: Vec2,
}

impl MoveCmd {
    pub fn new(from: Vec2, to: Vec2) -> MoveCmd {
        MoveCmd { from, to }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromoteCmd {
    from: Vec2,
    to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Cmd {
    Move(MoveCmd),
    Promote(PromoteCmd),
}
