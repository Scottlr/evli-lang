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

impl Parser for LexemeParser {
    fn parse(&self, sliding_window: &mut SlidingWindow) -> Token {
        Token::AwaitKeyword
    }
}


impl LexemeParser {
    pub fn new() -> LexemeParser {
        LexemeParser {
            token_parser: TokenParser,
            complex_token_parser: ComplexTokenParser
        }
    }
    fn parse_complex(sliding_window: &SlidingWindow) -> Token{

    }

    fn is_complex(token: Token) -> Token {

    }
}

