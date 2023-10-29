use lexer;
use serde::{Deserialize, Serialize};
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

    pub fn between(from: Vec2, to: Vec2) -> Vec<Vec2> {
        if from == to {
            return Vec::new();
        }

        let mut res = Vec::new();
        let x_differ = to.x - from.x;
        let y_differ = to.y - from.y;

        if abs(x_differ) == abs(y_differ) {
            let x_step = x_differ / abs(x_differ);
            let y_step = y_differ / abs(y_differ);

            for i in 1..abs(x_differ) {
                res.push(Vec2::new(from.x + i * x_step, from.y + i * y_step));
            }
            return res;
        }

        if x_differ == 0 {
            let y_step = y_differ / abs(y_differ);
            for i in 1..abs(y_differ) {
                res.push(Vec2::new(from.x, from.y + i * y_step));
            }
            return res;
        }

        if y_differ == 0 {
            let x_step = x_differ / abs(x_differ);
            for i in 1..abs(x_differ) {
                res.push(Vec2::new(from.x + i * x_step, from.y));
            }
            return res;
        }

        Vec::new()
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

fn board_cells() -> Vec<Vec2> {
    let mut res = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            res.push(Vec2::new(x, y))
        }
    }
    res
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
    moved: bool,
}

impl Pawn {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Pawn {
            moved: false,
            base: BasePiece::new(x, y, camp, "pawn".to_string()),
        }
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);

        if relat_move.x == 0 && relat_move.y == 1 {
            if board.get_piece(to).is_some() {
                return Err("pawn can move to that pos, because there already has a piece");
            }
        } else if relat_move.x == 0 && relat_move.y == 2 {
            if self.moved == true {
                return Err("the pawn only move by 2 when first move");
            }

            if board.get_piece(to).is_some() {
                return Err("pawn can move to that pos, because there already has a piece");
            }

            for pos in Vec2::between(self.base.pos, to) {
                if board.get_piece(pos).is_some() {
                    return Err("blocked by some piece");
                }
            }
        } else if abs(relat_move.x) == 1 && relat_move.y == 1 {
            if let Some(p) = board.get_piece(to) {
                if p.get_base().is_camp(self.base.camp) {
                    return Err("can not eat the piece belong to same camp");
                }
            } else {
                return Err("pawn only move to that pos , because there is no opposite camp piece");
            }
        } else {
            return Err("pawn can not move like that");
        }

        board.move_piece(self.base.pos, to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct King {
    base: BasePiece,
    moved: bool,
}
impl King {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        King {
            moved: false,
            base: BasePiece::new(x, y, camp, "king".to_string()),
        }
    }
    pub fn is_regular_move(&self, rmove: Vec2) -> bool {
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

    pub fn is_castling(&self, to: Vec2, board: &ChessBoard) -> bool {
        if self.moved == true {
            return false;
        }

        let relat_move = self.base.relative_move(to);
        if relat_move.y != 0 {
            return false;
        }

        let mut possible_rook_pos_1 = to.clone();
        let mut possible_rook_pos_2 = to.clone();

        possible_rook_pos_1.x -= 1;
        possible_rook_pos_2.x += 1;

        let mut rook: Option<Rook> = None;
        if let Some(p) = board.get_piece(possible_rook_pos_1) {
            if let Piece::Rook(r) = p {
                rook = Some(r);
            }
        } else if let Some(p) = board.get_piece(possible_rook_pos_2) {
            if let Piece::Rook(r) = p {
                rook = Some(r);
            }
        }

        if rook.is_some() {
            if !rook.unwrap().moved {
                return true;
            }
        }
        false
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);
        if self.is_regular_move(relat_move) {
            return board.move_piece(self.base.pos, to);
        }
        if self.is_castling(to, board) {
            return board.move_piece(self.base.pos, to);
        }
        return Err("king can not move like that");
    }
}

fn abs(i: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        -1 * i
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Queen {
    base: BasePiece,
}

impl Queen {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Queen {
            base: BasePiece::new(x, y, camp, "quee".to_string()),
        }
    }

    pub fn is_legal_move(&self, rmove: Vec2) -> bool {
        if rmove.x == 0 && rmove.y == 0 {
            return false;
        }

        if rmove.x == 0 || rmove.y == 0 {
            return true;
        }

        if abs(rmove.x) == abs(rmove.y) {
            return true;
        }

        false
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);
        if !self.is_legal_move(relat_move) {
            return Err("queen can not move like that");
        }

        for pos in Vec2::between(self.base.pos, to) {
            if board.get_piece(pos).is_some() {
                return Err("blocked by some piece");
            }
        }

        board.move_piece(self.base.pos, to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bishop {
    base: BasePiece,
}

impl Bishop {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Self {
            base: BasePiece::new(x, y, camp, "Bish".to_string()),
        }
    }

    fn is_legal_move(&self, rmove: Vec2) -> bool {
        if rmove.x == 0 && rmove.y == 0 {
            return false;
        }

        if abs(rmove.x) == abs(rmove.y) {
            return true;
        }

        false
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);
        if !self.is_legal_move(relat_move) {
            return Err("king can not move like that");
        }
        for pos in Vec2::between(self.base.pos, to) {
            if board.get_piece(pos).is_some() {
                return Err("blocked by some piece");
            }
        }

        board.move_piece(self.base.pos, to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rook {
    base: BasePiece,
    moved: bool,
}

impl Rook {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Self {
            base: BasePiece::new(x, y, camp, "rook".to_string()),
            moved: false,
        }
    }

    pub fn is_legal_move(&self, rmove: Vec2) -> bool {
        if rmove.x == 0 && rmove.y == 0 {
            return false;
        }
        if rmove.x == 0 || rmove.y == 0 {
            return true;
        }
        false
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);
        if !self.is_legal_move(relat_move) {
            return Err("rook can not move like that");
        }
        for pos in Vec2::between(self.base.pos, to) {
            if board.get_piece(pos).is_some() {
                return Err("blocked by some piece");
            }
        }

        board.move_piece(self.base.pos, to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Knight {
    base: BasePiece,
}

impl Knight {
    fn new(x: i32, y: i32, camp: Camp) -> Self {
        Self {
            base: BasePiece::new(x, y, camp, "Knig".to_string()),
        }
    }

    fn is_legal_move(&self, rmove: Vec2) -> bool {
        if abs(rmove.x) == 2 && abs(rmove.y) == 1 {
            return true;
        }

        if abs(rmove.x) == 1 && abs(rmove.y) == 2 {
            return true;
        }

        false
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        let relat_move = self.base.relative_move(to);
        if !self.is_legal_move(relat_move) {
            return Err("king can not move like that");
        }
        board.move_piece(self.base.pos, to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Piece {
    Pawn(Pawn),
    King(King),
    Queen(Queen),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
}

impl Piece {
    pub fn get_base(&self) -> BasePiece {
        match self {
            Piece::Pawn(p) => p.base.clone(),
            Piece::King(p) => p.base.clone(),
            Piece::Queen(p) => p.base.clone(),

            Piece::Bishop(p) => p.base.clone(),
            Piece::Knight(p) => p.base.clone(),
            Piece::Rook(p) => p.base.clone(),
        }
    }

    pub fn change_pos(&mut self, newpos: Vec2) {
        match *self {
            Piece::Pawn(ref mut p) => {
                p.base.pos = newpos;
                p.moved = true;
            }
            Piece::King(ref mut p) => {
                p.base.pos = newpos;
                p.moved = true;
            }
            Piece::Rook(ref mut p) => {
                p.base.pos = newpos;
                p.moved = true;
            }

            Piece::Queen(ref mut p) => p.base.pos = newpos,

            Piece::Knight(ref mut p) => p.base.pos = newpos,
            Piece::Bishop(ref mut p) => p.base.pos = newpos,
        }
    }

    pub fn deal_move(&self, to: Vec2, board: &mut ChessBoard) -> Result<(), &'static str> {
        match self {
            Piece::Pawn(p) => p.deal_move(to, board),
            Piece::King(p) => p.deal_move(to, board),
            Piece::Queen(p) => p.deal_move(to, board),

            Piece::Bishop(p) => p.deal_move(to, board),
            Piece::Knight(p) => p.deal_move(to, board),
            Piece::Rook(p) => p.deal_move(to, board),
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
        match self.remove_piece(from) {
            Ok(mut p) => {
                p.change_pos(to);
                self.board.insert(to.to_string(), p);
                Ok(())
            }

            Err(x) => Err(x),
        }
    }

    fn remove_piece(&mut self, pos: Vec2) -> Result<Piece, &'static str> {
        if let Some(res) = self.board.get(pos.to_string().as_str()) {
            let p: Piece;
            p = res.clone();
            self.board.remove(&pos.to_string()).unwrap();
            Ok(p)
        } else {
            Err(ERR_PIECE_NOT_FOUND)
        }
    }

    fn get_piece(&self, pos: Vec2) -> Option<Piece> {
        if let Some(p) = self.board.get(pos.to_string().as_str()) {
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
            .insert_piece(Piece::King(King::new(3, 7, Camp::Black)));

        game.board
            .insert_piece(Piece::Pawn(Pawn::new(3, 1, Camp::White)));

        game.board
            .insert_piece(Piece::Pawn(Pawn::new(3, 6, Camp::Black)));

        game.board
            .insert_piece(Piece::Queen(Queen::new(4, 0, Camp::White)));

        game.board
            .insert_piece(Piece::Queen(Queen::new(4, 7, Camp::Black)));

        game.board
            .insert_piece(Piece::Rook(Rook::new(0, 0, Camp::White)));

        game.board
            .insert_piece(Piece::Rook(Rook::new(0, 7, Camp::Black)));

        game.board
            .insert_piece(Piece::Knight(Knight::new(1, 0, Camp::White)));

        game.board
            .insert_piece(Piece::Knight(Knight::new(1, 7, Camp::Black)));

        game.board
            .insert_piece(Piece::Bishop(Bishop::new(2, 0, Camp::White)));

        game.board
            .insert_piece(Piece::Bishop(Bishop::new(2, 7, Camp::Black)));

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

        if self.stage.winner.is_some() {
            println!("finished winner{:?}", self.stage.winner)
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

        for p in opposite_piece {
            let mut game_copy = self.clone();
            game_copy.stage.change_turn();
            let cmd = Cmd::Move(MoveCmd {
                from: p.get_base().pos,
                to: our_king.base.pos,
            });

            let res = game_copy.exec_cmd_pre(&cmd);
            if res.is_ok() {
                println!("your can not make your king be killed {:?}", p);
                return Err("your can not make your king be killed");
            } else {
            }
        }

        Ok(())
    }

    fn valid_cmds(&mut self) -> Vec<Cmd> {
        let mut cmds = Vec::new();

        let froms = board_cells();
        let tos = board_cells();

        for from in froms.clone() {
            for to in tos.clone() {
                let mut game_copy = self.clone();
                if game_copy.deal_move(from, to).is_ok() {
                    if game_copy.after_check_king_dangerous().is_ok() {
                        cmds.push(Cmd::Move(MoveCmd { from, to }));
                    }
                }
            }
        }

        cmds
    }

    fn exec_cmd_after(&mut self) -> Result<(), &'static str> {
        self.after_check_king_dangerous()?;
        self.stage.change_turn();
        let cmds = self.valid_cmds();
        if cmds.len() == 0 {
            self.stage.winner = Some(self.stage.turn.opposite());
        } else {
            println!("{:?}", cmds);
        }
        Ok(())
    }

    fn deal_move(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        self.deal_move_target_confirm(from, to)?;
        self.deal_move_turn(from)?;
        self.deal_move_piece(from, to)
    }

    fn deal_move_target_confirm(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        if self.board.get_piece(from).is_some() {
            if let Some(to) = self.board.get_piece(to) {
                if to.get_base().is_camp(self.stage.turn) {
                    return Err("can not eat the piece belong to same camp");
                }
            }

            return Ok(());
        } else {
            return Err("can not find the picked piece");
        }
    }

    fn deal_move_turn(&mut self, from: Vec2) -> Result<(), &'static str> {
        if self.stage.winner.is_some() {
            return Err("game finished");
        }
        if self.stage.is_promotion {
            return Err("promotion mode");
        }

        let piece = self.board.get_piece(from).unwrap();
        if piece.get_base().is_your_turn(&self.stage) {
            Ok(())
        } else {
            Err("not your turn")
        }
    }

    fn deal_move_piece(&mut self, from: Vec2, to: Vec2) -> Result<(), &'static str> {
        let piece = self.board.get_piece(from).unwrap();

        piece.deal_move(to, &mut self.board)
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
