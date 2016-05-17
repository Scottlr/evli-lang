use super::slidingwindow::SlidingWindow;
use super::syntax::*;

struct Lexer {
    source_code_window: SlidingWindow
}

impl Lexer {
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code)
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let tokens = vec![];
        tokens
    }
}