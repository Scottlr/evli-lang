pub static OPERATORS: &'static [char] = 
    &['=', '(', ')', '+', '-', '{', '}', ';', '>', '<', ',', ':', '*'];

pub struct Syntax;

impl Syntax {
    pub fn map_operator_token(phrase: &str) -> Token {
        match phrase {
            ";" => Token::SemiColon,
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "=" => Token::Equals,
            "{" => Token::OpenBrace,
            "}" => Token::CloseBrace,
            ">" => Token::RightAngle,
            "<" => Token::LeftAngle,
            "-" => Token::Hyphen,
            "," => Token::Comma,
            ":" => Token::Colon,
            "*" => Token::Asterix,
            "+" => Token::Plus,
            "/" => Token::ForwardSlash,
            "\\" => Token::BackSlash,
            _ => panic!("Didn't recognise phrase to parse to token...")
        }
    } 

    pub fn map_compound_operator_token(current_token: Token, next_token: Token) -> Option<Token> {
        match (current_token, next_token) {
            (Token::Plus, Token::Equals) => Some(Token::PlusEquals),
            (Token::Hyphen, Token::Equals) => Some(Token::MinusEquals),
            (Token::Asterix, Token::Equals) => Some(Token::MultiplicationEquals),
            (Token::BackSlash, Token::Equals) => Some(Token::DivideEquals),
            _ => None
        }
    }
}


pub enum TokenType {
    Operator,
    CompoundOpeartor
}

pub enum Token {
    SemiColon,
    OpenParen,
    CloseParen,
    Equals,
    OpenBrace,
    CloseBrace,
    RightAngle,
    LeftAngle,
    Hyphen,
    Comma,
    Colon,
    Asterix,
    Plus,
    PlusEquals,
    MinusEquals,
    MultiplicationEquals,
    DivideEquals,
    BackSlash,
    ForwardSlash,
}
