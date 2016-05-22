use super::super::slidingwindow::SlidingWindow;
use super::super::token::*;

pub struct KeywordSyntaxParser;

impl KeywordSyntaxParser {
    pub fn parse_syntax(&self, source_code: &SlidingWindow) -> Token {
        Token::AwaitKeyword
       // while source_code.
    }

    fn map_keyword(phrase: &str) -> Option<Token> {
        match phrase {
            "await" => Some(Token::AwaitKeyword),
            "func" => Some(Token::FuncKeyword),
            "pub" => Some(Token::PublicModifierKeyword),
            "int" => Some(Token::IntKeyword),
            "float" => Some(Token::FloatKeyword),
            "string" => Some(Token::StringKeyword),
            _ => None
        }
    }

    fn valid_character(&self, phrase: char) -> bool {
        //let attempted_parse = phrase.to_digit(10);
        //match attempted_parse {
          //  Some(_) =>
        //}
        true
    }
}