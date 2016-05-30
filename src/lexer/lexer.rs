use super::slidingwindow::SlidingWindow;
use super::syntax::syntaxparser::SyntaxParser;
use super::token::Token;



pub struct Lexer {
    source_code_window: SlidingWindow,
    syntaxparser: SyntaxParser,
}

impl Lexer {    
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code),
            syntaxparser: SyntaxParser
        }
    }

    //Produce a vector of tokens with a given source code obtained from the constructor.u
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.source_code_window.is_eof() {
            self.syntaxparser.parse(&source_code_window);
        }
        vec![]
    }
    
}