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
        let mut token = self.token_parser.parse(sliding_window);
        if self.is_complex(&token) {
            token = self.parse_complex(sliding_window);
        }
        token
    }

    fn parse_complex(&self, sliding_window: &mut SlidingWindow) -> Token {
        self.complex_token_parser.parse(sliding_window)
    }   

    fn is_complex(&self,token: &Token) -> bool {
        match *token {
            Token::QoutationMark | Token::StartOfIdentifierOrKeyword 
                => true,
            _   => false
        }
    }
}

