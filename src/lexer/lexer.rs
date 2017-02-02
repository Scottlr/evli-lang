use super::slidingwindow::SlidingWindow;
use super::parsers::LexemeParser;
use super::token::{ Token, TokenKind };

pub struct Lexer;
impl Lexer {    
    //Produce a vector of tokens with a given source code obtained from the constructor.u
    pub fn tokenize(&self, source_code: &str) -> Vec<Token> {
        let lexeme_parser = LexemeParser::new();
        let mut source_code_window = SlidingWindow::new(source_code);
        let mut tokens = vec![];
        while source_code_window.can_peek() {
            let lexed_token = lexeme_parser.parse(&mut source_code_window);
            if !self.is_ignorable(&lexed_token.kind) {
                tokens.push(lexed_token);
            }
           
        }
        tokens
    }

    fn is_ignorable(&self, token: &TokenKind) -> bool {
        match *token {
            TokenKind::Whitespace | TokenKind::NewLine
                => true,
            _   => false
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::Lexer;
    use super::super::token::TokenKind;
    #[test]
    fn tokenize_standardcode_returnsCorrectNumberOfTokens() {
        let source_code = 
        "pub func() -> i32 {
            i32++;
        }";
            
        let text_lexer = Lexer;
        let tokens = text_lexer.tokenize(&source_code);
        assert_eq!(tokens.len(), 10);
    }

    #[test]
    fn tokenize_standardtokensaftercomplex_returnsCorrectNumberOfTokens() {
        let source_code = "pub func++;()";
        let text_lexer = Lexer;

        let tokens = text_lexer.tokenize(&source_code);

        assert_eq!(tokens.len(), 5);
    }

    // #[test]
    // fn tokenize_checkLastTokenIsParsed_returnsLastToken() {
    //     let source_code = "pub func++;()";
    //     let text_lexer = Lexer;
    //     let expected = TokenKind::CloseParen;

    //     let actualLastToken = text_lexer.tokenize(&source_code).pop();
        
    //     assert_eq!(expected, actualLastToken.unwrap().kind);
    // }
}
