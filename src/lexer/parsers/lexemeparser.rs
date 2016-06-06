use super::super::slidingwindow::SlidingWindow;
use super::tokenparser::TokenParser;
use super::complextokenparser::ComplexTokenParser;
use super::super::token::Token;

pub trait Parser {
    fn parse(&self, sliding_window: &mut SlidingWindow) -> Token;
}

pub struct LexemeParser {
    token_parser: TokenParser,
    complex_token_parser: ComplexTokenParser
}

impl LexemeParser {
    pub fn new() -> LexemeParser {
        LexemeParser {
            token_parser: TokenParser,
            complex_token_parser: ComplexTokenParser
        }
    }

    pub fn parse(&self, sliding_window: &mut SlidingWindow) -> Token {
        let is_complex = self.complex_token_parser.is_complex(sliding_window.current_character());
        match is_complex {
            true => self.complex_token_parser.parse(sliding_window),
            false => self.token_parser.parse(sliding_window)
        }        
    }
}

