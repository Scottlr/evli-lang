use super::super::slidingwindow::SlidingWindow;
use super::super::token::Token;


pub trait Parser {
    fn parse(sliding_window: &mut SlidingWindow) -> Token;
}


pub struct SyntaxParser;

impl SyntaxParser {
    pub fn parse_complex(sliding_window: &SlidingWindow) -> Token{

    }

    pub fn parse(sliding_window: &str) -> Token {

    }
}