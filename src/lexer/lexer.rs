use super::slidingwindow::SlidingWindow;
use super::syntax::*;

pub struct Lexer {
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
        let mut tokens = vec![];
        while !self.source_code_window.is_eof() {
            let character = self.source_code_window.advance_char();
            let mut token = self.syntax.map_token(&character.to_string());
            let token_type = self.syntax.get_token_type(token);

            match token_type {
                TokenType::Operator => {
                    let next_character = self.source_code_window.peek().to_string();
                    let next_token = self.syntax.map_token(&next_character);
                    if next_token == Token::Equals {
                        token = self.syntax.map_compound_token(token, next_token);
                    }
                },
                TokenType::Identifier => {
                    //Loop logic to white space?
                }
                _ => {}
            }
            tokens.push(token);
        }
        tokens
    }
    
}