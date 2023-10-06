use lexer::Lexer;

fn main() {
    println!("Hello, world!");
    let mut lexer = Lexer::new();
    lexer.add_keyword("new");
    lexer.add_keyword("for");
    lexer.add_keyword("while");
    lexer.add_keyword("fn");
    lexer.add_keyword("let");
    lexer.tokenize(
        "'asdfds',forlet , , while我 fn new new 12 011 11.2 12;  11.33( ( stu ) newstu.new=11+12.0*c/d",
    );
    println!("{:?}", lexer);
}
