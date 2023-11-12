#[derive(Debug)]
pub enum TokenType {
    Let,
    Equal,
    Identifier,
    Number,
    LParen,
    RParen,
    String,
    BinaryOp
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenType
}
pub struct Lexer {
    source: Vec<char>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source: source.chars().collect() }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut s = self.source.clone();
        while !s.is_empty() {
            if s[0] == '(' {
                tokens.push(Token { value: s.remove(0).to_string(), kind: TokenType::LParen });
            } else if s[0] == ')' {
                tokens.push(Token { value: s.remove(0).to_string(), kind: TokenType::RParen });
            } else if s[0] == '=' {
                tokens.push(Token { value: s.remove(0).to_string(), kind: TokenType::Equal });
            } else if 
            s[0] == '+' ||
            s[0] == '-' ||
            s[0] == '/' ||
            s[0] == '*' {
                tokens.push(Token { value: s.remove(0).to_string(), kind: TokenType::BinaryOp });
            } else if s[0] == '"' {
                let mut res_str = String::new();
                s.remove(0);
                while s[0] != '"' && !s.is_empty() {
                    res_str.push(s.remove(0));
                }
                s.remove(0);
                tokens.push(Token { value: res_str, kind: TokenType::String });
            } else if s[0].is_alphabetic() {
                let mut res_str = String::new();
                while s[0].is_alphabetic() && !s.is_empty() {
                    res_str.push(s.remove(0));
                }
                if res_str == "let" {
                    tokens.push(Token { value: res_str, kind: TokenType::Let });
                } else {
                    tokens.push(Token { value: res_str, kind: TokenType::Identifier });
                }
            } else if s[0].is_numeric() {
                let mut num = String::new();
                while s[0].is_numeric() && !s.is_empty() {
                    num.push(s.remove(0));
                }
                tokens.push(Token { value: num, kind: TokenType::Number });
            } else {
                s.remove(0);
            }

        }
        tokens
    }
}