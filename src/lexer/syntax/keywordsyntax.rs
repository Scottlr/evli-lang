use super::super::slidingwindow::SlidingWindow;
use super::super::token::*;

pub struct KeywordSyntaxParser;

impl KeywordSyntaxParser {
    pub fn parse_syntax(&self, source_code: &SlidingWindow) -> Token {
        while valid_character(*source_code.peek()) && !*source_code.is_eof() {

        }
        Token::AwaitKeyword
       // while source_code.P
    }

    fn map_keyword(self, phrase: &str) -> Option<Token> {
        match phrase {
            "await" =>  Some(Token::AwaitKeyword),
            "func" =>   Some(Token::FuncKeyword),
            "pub" =>    Some(Token::PublicModifierKeyword),
            "int" =>    Some(Token::IntKeyword),
            "float" =>  Some(Token::FloatKeyword),
            "string" => Some(Token::StringKeyword),
            _ => None
        }
    }

    fn valid_character(self, phrase: char) -> bool {
        let lowered_phrase = phrase.to_lowercase().next().unwrap();
        match lowered_phrase {
             'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |
             'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |
             'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
             'v' | 'w' | 'x' | 'y' | 'z' | '-' | '_'
                => true,
            _   => false
        }
        
    }
}