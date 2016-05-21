use super::super::slidingwindow::SlidingWindow;
use super::super::token::*;

pub struct KeywordSyntaxParser;

impl KeywordSyntaxParser {
    pub fn parse_syntax(&self, source_code: &SlidingWindow) -> Token {

    }

    fn map_keyword(phrase: &str) -> Option<Token> {
        match phrase {
            "await" => Token::AwaitKeyword,
            "func" => Token::FuncKeyword,
            "pub" => Token::PublicModifierKeyword,
            "int" => Token::IntKeyword,
            "float" => Token::FloatKeyword,
            "string" => Token::StringKeyword,
            _ => None
        }
    }
}