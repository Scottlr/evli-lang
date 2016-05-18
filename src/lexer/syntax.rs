pub static OPERATORS: &'static [char] = 
    &['=', '(', ')', '+', '-', '{', '}', ';', '>', '<', ',', ':', '*'];

pub struct Syntax;

impl Syntax {

    pub fn map_token(&self, phrase: &str) -> Token {
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

    pub fn get_token_type(&self, token: Token) -> TokenType {
        match token {
            Token::Plus |
            Token::Hyphen |
            Token::RightAngle |
            Token::LeftAngle |
            Token::Asterix |
            Token::ForwardSlash |
            Token::BackSlash 
                => TokenType::Operator,
                _ => panic!("Unknown...")
        }
    }

    pub fn map_compound_token(&self, current_token: Token, next_token: Token) -> Token {
        match (current_token, next_token) {
            (Token::Plus, Token::Equals) => Token::PlusEquals,
            (Token::Hyphen, Token::Equals) => Token::MinusEquals,
            (Token::Asterix, Token::Equals) => Token::MultiplicationEquals,
            (Token::BackSlash, Token::Equals) => Token::DivideEquals,
            _ => panic!("Unrecognised set of tokens!")
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    Operator,
    CompoundOpeartor
}

#[derive(PartialEq, Copy, Clone)]
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
