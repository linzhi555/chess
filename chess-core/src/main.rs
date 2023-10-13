use chess_core::{Game,Stage,Cmd,MoveCmd,PromoteCmd,Vec2};
use lexer::{Lexer, Token};
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

//fn main() {
//    let mut game = Game::new();
//    println!("{}",game.to_str());
//
//
//    println!("{:?}", game);
//    let mut i = -1;
//    loop {
//        i += 1;
//        if i == 10 {
//            game.stage = Stage::BlackWin;
//        }
//
//        let s = get_line();
//        let c = parse_command(&s);
//        if c.is_err() {
//            println!("{:?}", c);
//            continue;
//        }
//        let c = c.unwrap();
//        println!("{:?}", c);
//
//        let oldstate = game.clone();
//        let res = game.pre_check(&c);
//        if res != true {
//            game = oldstate;
//            continue;
//        }
//
//        let res = game.after_check(&c);
//
//        if res != true {
//            game = oldstate;
//            continue;
//        }
//
//        println!("{:?}", game);
//        match game.stage {
//            Stage::WhiteWin => {
//                println!("WhiteWin! the game is over");
//                break;
//            }
//            Stage::BlackWin => {
//                println!("Black Win! the game is over");
//                break;
//            }
//            _ => {}
//        }
//    }
//    println!("Hello, world!");
//}
