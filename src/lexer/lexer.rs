use super::slidingwindow::SlidingWindow;
use super::parsers::LexemeParser;
use super::parsers::Parser;
use super::token::Token;

pub struct Lexer {
    source_code_window: SlidingWindow,
    lexparser: LexemeParser,
}

impl Lexer {    
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code),
            lexparser: LexemeParser::new()
        }
    }

    //Produce a vector of tokens with a given source code obtained from the constructor.u
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.source_code_window.is_eof() {
            self.lexparser.parse(&mut self.source_code_window);
        }
        vec![]
    }
    
}