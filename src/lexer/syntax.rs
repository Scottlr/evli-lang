pub struct Syntax;

impl Syntax {

    pub fn map_token(&self, phrase: &str) -> Token {
        match phrase {
            ";" => Token::SemiColon,    "(" => Token::OpenParen,
            ")" => Token::CloseParen,   "=" => Token::Equals,
            "{" => Token::OpenBrace,    "}" => Token::CloseBrace,
            ">" => Token::RightAngle,   "<" => Token::LeftAngle,
            "-" => Token::Hyphen,       "," => Token::Comma,
            ":" => Token::Colon,        "*" => Token::Asterix,
            "+" => Token::Plus,         "/" => Token::ForwardSlash,
            "\\" => Token::BackSlash,
            _ if is_character(phrase) => Token::Character,
            _ => panic!("Didn't recognise phrase to parse to token... {}", phrase)
        }
    } 

    pub fn get_token_type(&self, token: Token) -> TokenType {
        match token {
            Token::Plus         | Token::Hyphen         |
            Token::RightAngle   | Token::LeftAngle      |
            Token::Asterix      | Token::ForwardSlash   |
            Token::BackSlash 
                => TokenType::Operator,
            _ if is_character( )
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

    pub fn is_character(&self, character: char) -> bool {
        let converted = character.to_digit(10);
        match converted {
            Some(_) => true,
            None => false
        }
    }
}

