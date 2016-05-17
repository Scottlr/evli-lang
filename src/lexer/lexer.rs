use super::slidingwindow::SlidingWindow;

struct Lexer {
    source_code_window: SlidingWindow
}

impl Lexer {
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code)
        }
    }
}