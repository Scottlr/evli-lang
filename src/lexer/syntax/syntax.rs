use super::super::token::*;


pub struct Syntax(Token, TokenType);
pub struct SyntaxParser;

impl SyntaxParser {
    fn map_token(&self, phrase: &str) -> Token {
        match phrase {
            ";" => Token::SemiColon,    "(" => Token::OpenParen,
            ")" => Token::CloseParen,   "=" => Token::Equals,
            "{" => Token::OpenBrace,    "}" => Token::CloseBrace,
            ">" => Token::RightAngle,   "<" => Token::LeftAngle,
            "-" => Token::Hyphen,       "," => Token::Comma,
            ":" => Token::Colon,        "*" => Token::Asterix,
            "+" => Token::Plus,         "/" => Token::ForwardSlash,
            "\\" => Token::BackSlash,   "\"" => Token::QoutationMark,
            _ => panic!("Didn't recognise phrase to parse to token... {}", phrase)
        }
    } 

    fn get_token_type(&self, token: Token) -> TokenType {
        match token {
            Token::Plus         | Token::Hyphen         |
            Token::RightAngle   | Token::LeftAngle      |
            Token::Asterix      | Token::ForwardSlash   |
            Token::BackSlash 
                => TokenType::Operator,
            Token::Character
                => TokenType::Identifier,
            _ => panic!("Unknown...")

        }
    }

    pub fn parse(&self, phrase: &str) -> Syntax {
        let token = map_token(phrase);
        let tokentype = get_token_type(token);
        Syntax(token, tokentype) 
    }

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

