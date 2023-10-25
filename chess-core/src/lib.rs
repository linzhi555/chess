use lexer;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Camp {
    White,
    Black,
}

impl Camp {
    fn opposite(&self) -> Camp {
        match *self {
            Camp::White => Camp::Black,
            Camp::Black => Camp::White,
        }
    }
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

    pub fn is_camp(&self, camp: Camp) -> bool {
        if self.camp == camp {
            true
        } else {
            false
        }
    }

    pub fn is_your_turn(&self, s: &Stage) -> bool {
        match s.turn {
            Camp::White => {
                if self.camp == Camp::White {
                    true
                } else {
                    false
                }
            }
            Camp::Black => {
                if self.camp == Camp::White {
                    false
                } else {
                    true
                }
            }
        }
    }

    pub fn relative_move(&self, newpos: Vec2) -> Vec2 {
        if self.camp == Camp::White {
            Vec2::new(newpos.x - self.pos.x, newpos.y - self.pos.y)
        } else {
            Vec2::new(self.pos.x - newpos.x, self.pos.y - newpos.y)
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
    pub fn is_legal_move(rmove: Vec2) -> bool {
        if rmove.x > 1 || rmove.x < -1 {
            return false;
        }

        if rmove.y > 1 || rmove.y < -1 {
            return false;
        }

        if rmove.x == 0 && rmove.y == 0 {
            return false;
        }

        true
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

    pub fn change_pos(&mut self, newpos: Vec2) {
        match *self {
            Piece::Pawn(ref mut p) => p.base.pos = newpos,
            Piece::King(ref mut p) => p.base.pos = newpos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stage {
    turn: Camp,
    is_promotion: bool,
    winner: Option<Camp>,
}

impl Stage {
    fn change_turn(&mut self) {
        match self.clone().turn {
            Camp::Black => {
                self.turn = Camp::White;
            }

            Camp::White => {
                self.turn = Camp::Black;
            }
        }
    }
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
        let mut p: Piece;
        if let Some(res) = self.board.get(from.to_string().as_str()) {
            p = res.clone();
            p.change_pos(to);
        } else {
            return Err(ERR_PIECE_NOT_FOUND);
        }

        self.board.remove(&from.to_string()).unwrap();
        self.board.insert(to.to_string(), p);

        Ok(())
    }

    fn get_piece(&mut self, pos: Vec2) -> Option<Piece> {
        if let Some(p) = self.board.get_mut(pos.to_string().as_str()) {
            Some(p.clone())
        } else {
            None
        }
    }

    fn get_king_of_camp(&self, camp: Camp) -> King {
        for ele in &self.board {
            match ele.1 {
                Piece::King(p) => {
                    if p.base.is_camp(camp) {
                        return p.clone();
                    }
                }

                _ => {}
            }
        }
        panic!("king not found")
    }

    fn get_piece_of_camp(&self, camp: Camp) -> Vec<Piece> {
        let mut pieces = Vec::new();
        for ele in &self.board {
            let p = ele.1;
            if p.get_base().is_camp(camp) {
                pieces.push(p.clone())
            }
        }
        pieces
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
            stage: Stage {
                turn: Camp::White,
                is_promotion: false,
                winner: None,
            },
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
        let game_backup = self.clone();

        let res = self.exec_cmd_pre(c);
        if res.is_err() {
            *self = game_backup;
            return res;
        }

        let res = self.exec_cmd_after();
        if res.is_err() {
            *self = game_backup;
            return res;
        }

        Ok(())
    }

    fn exec_cmd_pre(&mut self, c: &Cmd) -> Result<(), &'static str> {
        match c {
            Cmd::Move(x) => self.deal_move(x.from, x.to),

            Cmd::Promote(_) => Ok(()),
        }
    }

    fn after_check_king_dangerous(&self) -> Result<(), &'static str> {
        let our_king = self.board.get_king_of_camp(self.stage.turn);
        let opposite_piece = self.board.get_piece_of_camp(self.stage.turn.opposite());
        println!("{:?}", our_king);
        println!("{:?}", opposite_piece);

        for p in opposite_piece {
            let mut game_copy = self.clone();
            game_copy.stage.change_turn();
            let cmd = Cmd::Move(MoveCmd {
                from: p.get_base().pos,
                to: our_king.base.pos,
            });

            let res = game_copy.exec_cmd_pre(&cmd);
            if res.is_ok() {
                println!("{:?}", game_copy);
                return Err("your can not make your king be killed");
            } else {
                println!("{:?}", res)
            }
        }

        Ok(())
    }

    fn exec_cmd_after(&mut self) -> Result<(), &'static str> {
        self.after_check_king_dangerous()?;
        self.stage.change_turn();
        Ok(())
    }

    fn deal_move(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        if let Some(_) = self.board.get_piece(from) {
            self.deal_move_turn(from)?;
            self.deal_move_piece(from, to)?;
            self.board.move_piece(from, to)
        } else {
            return Err("piece can not find");
        }
    }

    fn deal_move_turn(&mut self, from: Vec2) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from).unwrap();
        if piece.get_base().is_your_turn(&self.stage) {
            Ok(())
        } else {
            Err("not your turn")
        }
    }

    fn deal_move_piece(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from).unwrap();
        match piece {
            Piece::Pawn(_) => {
                if piece.get_base().relative_move(to) == Vec2::new(0, 1) {
                    return Ok(());
                } else {
                    return Err("pawn can not move like that");
                }
            }

            Piece::King(_) => {
                let relat_move = piece.get_base().relative_move(to);
                if King::is_legal_move(relat_move) {
                    return Ok(());
                } else {
                    return Err("king can not move like that");
                }
            }
        }
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
