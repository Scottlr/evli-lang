use super::super::token::Token;
use super::super::slidingwindow::SlidingWindow;
use super::syntaxparser::Parser;
pub struct TokenParser;

impl Parser for TokenParser {
    fn parse(source_code: &mut SlidingWindow) -> Token {
        let phrase = source_code.current_character();
        match phrase {
            ';' => Token::SemiColon,    '(' => Token::OpenParen,
            ')' => Token::CloseParen,   '=' => Token::Equals,
            '{' => Token::OpenBrace,    '}' => Token::CloseBrace,
            '>' => Token::RightAngle,   '<' => Token::LeftAngle,
            '-' => Token::Hyphen,       ',' => Token::Comma,
            ':' => Token::Colon,        '*' => Token::Asterix,
            '+' => Token::Plus,         '/' => Token::ForwardSlash,
            '\\' => Token::BackSlash,   '\"' => Token::QoutationMark,
            _ => panic!("Didn't recognise phrase to parse to token... {}", phrase)
        }
    } 
 
}


impl TokenParser {
    pub fn map_compound_token(&self, current_token: Token, next_token: Token) -> Token {
        match (current_token, next_token) {
            (Token::Plus, Token::Equals) => Token::PlusEquals,
            (Token::Hyphen, Token::Equals) => Token::MinusEquals,
            (Token::Asterix, Token::Equals) => Token::MultiplicationEquals,
            (Token::BackSlash, Token::Equals) => Token::DivideEquals,
            _ => panic!("Invalid combinations for compound token...")
        }
        
    }
}
