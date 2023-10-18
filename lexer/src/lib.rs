#[derive(Debug)]
pub enum Token {
    LParentheses,
    RParentheses,
    Simicolon,
    Dot,
    Plus,
    Minus,
    Slash,
    Multi,
    Equal,
    Comma,
    Int(i32),
    Float(f32),
    Keyword(String),
    Symbol(String),
    String(String),
}

#[derive(Debug)]
pub struct Lexer {
    keywords: Vec<String>,
    pub result: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            keywords: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn to_token_vec(s: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        lexer.tokenize(s);
        lexer.result
    }

    pub fn add_keyword(&mut self, k: &str) {
        self.keywords.push(k.to_string());
    }

    fn is_keyword(&self, s: &str) -> bool {
        for keyword in self.keywords.iter() {
            if keyword.as_str() == s {
                return true;
            }
        }

        return false;
    }

    pub fn tokenize(&mut self, input: &str) {
        let mut stream = input.chars().peekable();
        loop {
            let c: char;
            if stream.peek().is_none() {
                break;
            } else {
                c = stream.next().unwrap()
            }

            match c {
                '(' => {
                    self.result.push(Token::LParentheses);
                    continue;
                }
                ')' => {
                    self.result.push(Token::RParentheses);
                    continue;
                }
                '.' => {
                    self.result.push(Token::Dot);
                    continue;
                }
                ';' => {
                    self.result.push(Token::Simicolon);
                    continue;
                }
                '+' => {
                    self.result.push(Token::Plus);
                    continue;
                }
                '-' => {
                    self.result.push(Token::Minus);
                    continue;
                }
                '/' => {
                    self.result.push(Token::Slash);
                    continue;
                }
                '*' => {
                    self.result.push(Token::Multi);
                    continue;
                }
                '=' => {
                    self.result.push(Token::Equal);
                    continue;
                }
                ',' => {
                    self.result.push(Token::Comma);
                    continue;
                }

                _ => {}
            }

            if c.is_numeric() {
                let mut buffer = String::new();
                buffer.push(c);
                loop {
                    if stream.peek().is_none() {
                        break;
                    } else if stream.peek().unwrap().is_alphanumeric()
                        || stream.peek().unwrap().clone() == '.'
                    {
                        let c = stream.next().unwrap();
                        buffer.push(c);
                    } else {
                        break;
                    }
                }
                if let Ok(i) = buffer.parse::<i32>() {
                    self.result.push(Token::Int(i));
                    continue;
                } else if let Ok(f) = buffer.parse::<f32>() {
                    self.result.push(Token::Float(f));
                } else {
                    panic!("numeric parse error: {}", buffer)
                }
            }

            if c.is_alphabetic() {
                let mut buffer = String::new();
                buffer.push(c);
                loop {
                    if stream.peek().is_none() {
                        break;
                    } else if stream.peek().unwrap().is_alphanumeric() {
                        let c = stream.next().unwrap();
                        buffer.push(c);
                    } else {
                        break;
                    }
                }
                if self.is_keyword(&buffer) {
                    self.result.push(Token::Keyword(buffer))
                } else {
                    self.result.push(Token::Symbol(buffer))
                }
            }

            if c == '\'' {
                let mut buffer = String::new();
                loop {
                    if stream.peek().is_none() {
                        panic!("string tokenize error")
                    } else if stream.peek().unwrap().clone() == '\'' {
                        stream.next();
                        break;
                    } else {
                        let c = stream.next().unwrap();
                        buffer.push(c);
                    }
                }
                self.result.push(Token::String(buffer))
            }
        }
    }
}
