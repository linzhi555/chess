use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Node {
    x:u8,
    y:u8,
    team:String,
    s:String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Board{
    nodes:Vec<Node>

}

impl Board{
    fn new()->Self{

        let mut nodes = Vec::new();
        nodes.push(Node{
        x:1,
        y:1,
        team:"white".to_string(),
        s:"white".to_string(),
        });

        nodes.push(Node{
        x:1,
        y:3,
        team:"black".to_string(),
        s:"black".to_string(),
        });


        return Board {nodes }


    }
    
}


fn main() {
    println!("Hello, world!");
    let b = Board::new();
    let s = serde_json::to_string(&b).unwrap();
    println!("{}",s);
    let b2:Board = serde_json::from_str(&s).unwrap();
    println!("{:?}",b2)
}
