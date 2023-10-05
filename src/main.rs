#[derive(Debug)]
#[derive(Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(Clone)]
enum Piece {
    Pawn(Pawn),
    King(King),
    None,
}

#[derive(Debug)]
#[derive(Clone)]
enum Stage {
    WhiteTurn,
    BlackTurn,
    WhitePromotion,
    BlackPromotion,
    WhiteWin,
    BlackWin,
}

#[derive(Debug)]
#[derive(Clone)]
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

    fn pre_check(&mut self,c: &str) -> bool{


        true
    }


    fn after_check(&mut self,c:&str) -> bool{
        true
    }


}

fn get_command() -> String{
    String::from("asdfasd")
}


fn main() {
    let mut game = Game::new();

    let mut i = -1;
    loop {
        i += 1;
        if i == 10 {
            game.stage = Stage::BlackWin;
        }
        
        println!("{:?}",game);
        
        let c = get_command();
        let oldstate=game.clone(); 
        let res = game.pre_check(&c);
        if res != true {
            game=oldstate;
            continue;
        }

        let res = game.after_check(&c);

        if res != true {
            game=oldstate;
            continue;
        }



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
