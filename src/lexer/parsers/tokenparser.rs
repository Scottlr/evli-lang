use super::super::token:: { Token, TokenKind };
use super::super::slidingwindow::SlidingWindow;
use super::lexemeparser::Parser;


pub struct TokenParser;

impl Parser for TokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        println!("trying to parse character: {}", source_code.current_character());
        
        //A token should always be returned... wishful thinking?
        let mut tokenkind = self.map_token(source_code.current_character()).unwrap();
        if source_code.can_peek() {
            let next_character = source_code.peek();
            if let Some(parsed_next_token) = self.map_token(next_character) {
                if let Some(parsed_compound_tokenkind) = self.map_compound_token(tokenkind.clone(), parsed_next_token) {
                    tokenkind = parsed_compound_tokenkind;
                    source_code.advance();
                }
            }
        }   
        source_code.advance();
        Token::construct(tokenkind, source_code)
    } 
}

impl TokenParser {
    pub fn map_compound_token(&self, current_token: TokenKind, next_token: TokenKind) -> Option<TokenKind> {
        match (current_token, next_token) {
            (TokenKind::Plus, TokenKind::Equals) =>         Some(TokenKind::PlusEquals),
            (TokenKind::Hyphen, TokenKind::Equals) =>       Some(TokenKind::MinusEquals),
            (TokenKind::Asterix, TokenKind::Equals) =>      Some(TokenKind::MultiplicationEquals),
            (TokenKind::ForwardSlash, TokenKind::Equals) => Some(TokenKind::DivideEquals),
            (TokenKind::Equals, TokenKind::Equals) =>       Some(TokenKind::ConditionalEquals),
            (TokenKind::Plus, TokenKind::Plus) =>           Some(TokenKind::IncrementOperator),
            (TokenKind::Hyphen, TokenKind::Hyphen) =>       Some(TokenKind::DecrementOperator),
            (TokenKind::Hyphen, TokenKind::RightAngle) =>   Some(TokenKind::PointerArrow),
            _ => None
        }
    }

    pub fn map_token(&self, character: char) -> Option<TokenKind> {
         match character {
            ';' =>  Some(TokenKind::SemiColon),     '(' =>  Some(TokenKind::OpenParen),
            ')' =>  Some(TokenKind::CloseParen),    '=' =>  Some(TokenKind::Equals),
            '{' =>  Some(TokenKind::OpenBrace),     '}' =>  Some(TokenKind::CloseBrace),
            '>' =>  Some(TokenKind::RightAngle),    '<' =>  Some(TokenKind::LeftAngle),
            '-' =>  Some(TokenKind::Hyphen),        ',' =>  Some(TokenKind::Comma),
            ':' =>  Some(TokenKind::Colon),         '*' =>  Some(TokenKind::Asterix),
            '+' =>  Some(TokenKind::Plus),          '/' =>  Some(TokenKind::ForwardSlash),
            '\\' => Some(TokenKind::BackSlash),     '\"' => Some(TokenKind::QoutationMark),
            ' ' =>  Some(TokenKind::Whitespace),    '\n' => Some(TokenKind::NewLine),
            '\r' => Some(TokenKind::CarraigeReturn),
            _ => None
        }
    }

}



#[cfg(test)]
mod tests {
    use super::TokenParser;
    use super::super::lexemeparser::Parser;
    use super::super::super::slidingwindow::SlidingWindow;
    use super::super::super::TokenKind::Token;
    use super::super::super::TokenKind::TokenKind;
   
    fn parser_helper(source: &str) -> TokenKind {
        let parser = TokenParser;
        let mut phrase = SlidingWindow::new(source);
        parser.parse(&mut phrase).kind
    }

    #[test]
    fn test_parser_singletokens() {
        assert_eq!(parser_helper("+"), TokenKind::Plus);
        assert_eq!(parser_helper("-"), TokenKind::Hyphen);
        assert_eq!(parser_helper("*"), TokenKind::Asterix);
        assert_eq!(parser_helper("{"), TokenKind::OpenBrace);
        assert_eq!(parser_helper("}"), TokenKind::CloseBrace);
        assert_eq!(parser_helper("\\"), TokenKind::BackSlash);
        assert_eq!(parser_helper("\""), TokenKind::QoutationMark);
        assert_eq!(parser_helper(" "), TokenKind::Whitespace);
        assert_eq!(parser_helper("\r"), TokenKind::CarraigeReturn);
        assert_eq!(parser_helper("\n"), TokenKind::NewLine);
    }

    #[test]
    fn test_parser_compoundtokens() {
        assert_eq!(parser_helper("+="), TokenKind::PlusEquals);
        assert_eq!(parser_helper("-="), TokenKind::MinusEquals);
        assert_eq!(parser_helper("*="), TokenKind::MultiplicationEquals);
        assert_eq!(parser_helper("/="), TokenKind::DivideEquals);
        assert_eq!(parser_helper("=="), TokenKind::ConditionalEquals);
        assert_eq!(parser_helper("++"), TokenKind::IncrementOperator);
        assert_eq!(parser_helper("--"), TokenKind::DecrementOperator);
        assert_eq!(parser_helper("->"), TokenKind::PointerArrow);
    }

    #[test]
    #[should_panic]
    fn test_compoundtokensparser_invalidsequenceshouldfail() {
        let parser = TokenParser;
        parser.map_compound_token(TokenKind::AsyncKeyword, TokenKind::AwaitKeyword);
        assert!(false);
    }


    #[test]
    #[should_panic]
    fn test_maptokenparser_invalidsequenceshouldfail() {
        let parser = TokenParser;
        parser.map_token('f');;
        assert!(false);
    }
}