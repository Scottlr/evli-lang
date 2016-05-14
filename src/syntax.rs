pub static OPERATORS: &'static [char] = 
    &['=', '(', ')', '+', '-', '{', '}', ';', '>', '<', ',', ':', '*'];

pub enum Token
{
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
}

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
        _ => panic!("Didn't recognise phrase to parse to token...")
    }
} 

pub fn map_compound_operator_token(current_token: Token, next_token: Token) -> Token {
    match (current_token, next_token) {
        (Token::Plus, Token::Equals) => Token::PlusEquals,
        _ => panic!("Unknown sequence of operators!")
    }
}