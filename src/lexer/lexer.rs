use super::slidingwindow::SlidingWindow;
use super::syntax::SyntaxParser;
use super::syntax::Syntax;
use super::token::*;



pub struct Lexer {
    source_code_window: SlidingWindow,
    syntaxparser: SyntaxParser
}

impl Lexer {    
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code),
            syntaxparser: SyntaxParser
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.source_code_window.is_eof() {
            let character = self.source_code_window.advance_char();
            let mut token = self.syntaxparser.map_token(&character.to_string());
            let token_type = self.syntaxparser.get_token_type(&token);


            //matching on types and tokens gives us the ability to match specific tokens
            //and still have the ability to neatly match all tokens of a specific 'type'
            match (token_type, token) {
                (TokenType::Operator, Token::QoutationMark) => {

                },
                (TokenType::Operator, _) => {
                    let next_character = self.source_code_window.peek().to_string();
                    let next_token = self.syntaxparser.map_token(&next_character);
                    if next_token == Token::Equals {
                        token = self.syntaxparser.map_compound_token(&token, &next_token);
                    }
                },
                (_, Token::Identifier(_)) => {
                    //Loop logic to white space?
                },
                (_, _) => {}
            }
            tokens.push(token);
        }
        tokens
    }
    
}