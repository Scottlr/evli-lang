use super::slidingwindow::SlidingWindow;
use super::parsers::LexemeParser;
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
        let mut tokens = vec![];
        while !self.source_code_window.is_eof() {
            let lexed_token = self.lexparser.parse(&mut self.source_code_window);
            println!("{:?}", lexed_token);
            tokens.push(lexed_token);
        }
        tokens
    }
    
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::super::token::Token;
    #[test]
    fn test_parser_keywords() {
        let source_code = 
        "pub fn() -> i32 {
            i32++
        }";
            
        let text_lexer = Lexer::new(source_code);
        let tokens = text_lexer.tokenize();
    }

}
