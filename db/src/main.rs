use lexer::{Lexer, Token};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, ErrorKind};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct DB {
    head: Vec<String>,
    tables: HashMap<String, Vec<Token>>,
}

impl DB {
    fn open(filepath: &str) -> Result<DB, ()> {
        let mut map = HashMap::new();
        let lines = read_lines(filepath);
        if lines.is_err() {
            return Err(());
        }
        let mut lines = lines.unwrap();

        let mut head = lines.next();

        if head.is_none() {
            return Err(());
        }

        let head = Lexer::to_token_vec(&head.unwrap().unwrap());
        let mut head_after = Vec::new();
        for item in head {
            if let Token::Symbol(s) = item {
                head_after.push(s)
            } else {
                return Err(());
            }
        }
        let head = head_after;

        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                let v = Lexer::to_token_vec(l.as_str());
                if let Token::Symbol(s) = v.get(0).unwrap().clone() {
                    map.insert(s, v.clone());
                }
            }
        }
        Ok(DB { head, tables: map })
    }

    fn flush(&self, output: &str) {
        let mut f = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)
            .unwrap();
        for i in self.head.iter() {
            write!(&mut f, "{} ", i).unwrap();
        }
        write!(&mut f, "\n").unwrap();
        for v in self.tables.values() {
            for item in v.iter() {
                match item {
                    Token::String(x) => write!(&mut f, "'{}' ", x).unwrap(),
                    Token::Symbol(s) => write!(&mut f, "{} ", s).unwrap(),
                    Token::Int(s) => write!(&mut f, "{} ", s).unwrap(),
                    Token::Float(s) => write!(&mut f, "{} ", s).unwrap(),
                    _ => {
                        panic!("asdf")
                    }
                }
            }
            write!(&mut f, "\n").unwrap();
        }
    }
    fn field_to_int(&self,field:&str) -> Option<usize> {
        let mut i:usize=0;
        for name in self.head.iter() {
            if name==field{
                return Some(i)
            }
            i += 1;
        }
        None
    }

    fn set(&mut self,key :&str,field:&str,value:Token) -> Result<(),()>{
        let index  = self.field_to_int(field);
        if index.is_none(){
            return Err(())
        }
        let index = index.unwrap();
        if let Some(x) = self.tables.get_mut(key){
            x.remove(index);
            x.insert(index, value);
            return Ok(())
        }

        Err(())
    }

    fn get(&mut self,key :&str,field:&str) -> Result<Token,()>{
        let index  = self.field_to_int(field);
        if index.is_none(){
            return Err(())
        }
        let index = index.unwrap();
        if let Some(x) = self.tables.get_mut(key){
            Ok(x.get(index).unwrap().clone())
        }else{
            Err(())
        }
    }
    fn insert(&mut self,key :&str,data:Vec<Token>) {
        self.tables.insert(key.to_string(), data);
    }


    fn deal_cmd(&mut self, s:&str) -> Result<(),()> {
        let tokens = Lexer::to_token_vec(s);
        self.get_deal(&tokens);
        self.insert_deal(&tokens);
        Ok(())
    }
    
    fn get_deal(&mut self,v:&Vec<Token>) {
        match  v.get(0).cloned() {
            Some(Token::Symbol(x)) => {
                if x.as_str() == "get" {
                    if let Some(Token::Symbol(key)) = v.get(1){
                        if let Some(Token::Symbol(field)) = v.get(2){
                            println!("{:?}",self.get(key, field));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn insert_deal(&mut self,v:&Vec<Token>) {
            if let Some(Token::Symbol(x)) = v.get(0).clone(){
                if x.as_str() == "insert" {
                    if let Some(Token::Symbol(key)) = v.get(1){
                        let mut after = v.clone();
                        after.remove(0);
                        self.insert(key, after)
                    }
                }
            }
    }

}




fn main() {
    let mut db = DB::open("./test.db").unwrap();
    println!("{:?}", db);
    db.set("a", "password", Token::String("aaaaaaaa".to_string())).unwrap();

    loop {
       let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        db.deal_cmd(buffer.as_str());
        db.flush("./test.db");
        println!("{:?}", db);
    }

    //println!("{:?}",db.get("b","age"));
}
