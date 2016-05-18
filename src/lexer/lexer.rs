use super::slidingwindow::SlidingWindow;
use super::syntax::*;

struct Lexer {
    source_code_window: SlidingWindow,
    syntax: Syntax
}

impl Lexer {
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code),
            syntax: Syntax
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let tokens = vec![];

        while !self.source_code_window.is_eof() {
            let character = self.source_code_window.advance_char();
            let token = self.syntax.map_token(&character.to_string());
            if self.syntax.is_operator(token) {

            }
        }
        tokens
    }
}