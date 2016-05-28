use super::slidingwindow::SlidingWindow;
use super::syntax::syntaxparser::SyntaxParser;
use super::token::*;



pub struct Lexer {
    source_code_window: SlidingWindow,
    syntaxparser: SyntaxParser,
}

impl Lexer {    
    pub fn new(source_code: &str) -> Lexer {
        Lexer {
            source_code_window: SlidingWindow::new(source_code),
            syntaxparser: SyntaxParser,
        }
    }

    //Produce a vector of tokens with a given source code obtained from the constructor.u
    pub fn tokenize(&mut self) -> Vec<Token> {
      /*  let mut tokens = vec![];
        while !self.source_code_window.is_eof() {
            let character = self.source_code_window.advance_char();
            let mut token = self.syntaxparser.map_token(&character.to_string());
         
            match token {
                //Match potential compound tokens? if not just add the token...
                Token::Plus | Token::Hyphen | Token::Asterix | Token::BackSlash => {
                    let next_character = self.source_code_window.peek().to_string();
                    let next_token = self.syntaxparser.map_token(&next_character);
                    if next_token == Token::Equals {
                        token = self.syntaxparser.map_compound_token(token.clone(), next_token.clone());
                    }
                },
                Token::StartOfIdentifierOrKeyword | Token::QoutationMark => {
                   
                },
                _ => { /*Should be trivial tokens, or unrecognised??*/ } 
            }
            tokens.push(token);
        }
        tokens
*/
vec![]
    }
    
}