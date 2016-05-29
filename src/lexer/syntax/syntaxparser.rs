use super::super::slidingwindow::SlidingWindow;
use super::tokenparser::TokenParser;
use super::complextokenparser::ComplexTokenParser;
use super::super::token::Token;

pub trait Parser {
    fn parse(&self, sliding_window: &mut SlidingWindow) -> Token;
}

pub struct SyntaxParser {
    token_parser: TokenParser,
    complex_token_parser: ComplexTokenParser
}

impl Parser for SyntaxParser {
    fn parse(&self, sliding_window: &mut SlidingWindow) -> Token {
        Token::AwaitKeyword
    }
}


impl SyntaxParser {
    pub fn parse_complex(sliding_window: &SlidingWindow) -> Token{

    }

    pub fn is_complex(token: Token) -> Token {

    }
}

