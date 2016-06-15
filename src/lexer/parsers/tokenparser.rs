use super::super::token::Token;
use super::super::slidingwindow::SlidingWindow;
use super::lexemeparser::Parser;


pub struct TokenParser;

impl Parser for TokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        println!("trying to parse character: {}", source_code.current_character());
        
        //A token should always be returned... wishful thinking?
        let mut token = self.map_token(source_code.current_character()).unwrap();
        if source_code.can_peek() {
            let next_character = source_code.peek();
            if let Some(parsed_next_token) = self.map_token(next_character) {
                if let Some(parsed_compound_token) = self.map_compound_token(token.clone(), parsed_next_token) {
                    token = parsed_compound_token;
                    source_code.advance();
                }
            }
        }   
        source_code.advance();
        token
    } 
}

impl TokenParser {
    pub fn map_compound_token(&self, current_token: Token, next_token: Token) -> Option<Token> {
        match (current_token, next_token) {
            (Token::Plus, Token::Equals) =>         Some(Token::PlusEquals),
            (Token::Hyphen, Token::Equals) =>       Some(Token::MinusEquals),
            (Token::Asterix, Token::Equals) =>      Some(Token::MultiplicationEquals),
            (Token::ForwardSlash, Token::Equals) => Some(Token::DivideEquals),
            (Token::Equals, Token::Equals) =>       Some(Token::ConditionalEquals),
            (Token::Plus, Token::Plus) =>           Some(Token::IncrementOperator),
            (Token::Hyphen, Token::Hyphen) =>       Some(Token::DecrementOperator),
            (Token::Hyphen, Token::RightAngle) =>   Some(Token::PointerArrow),
            _ => None
        }
    }

    pub fn map_token(&self, character: char) -> Option<Token> {
         match character {
            ';' =>  Some(Token::SemiColon),     '(' =>  Some(Token::OpenParen),
            ')' =>  Some(Token::CloseParen),    '=' =>  Some(Token::Equals),
            '{' =>  Some(Token::OpenBrace),     '}' =>  Some(Token::CloseBrace),
            '>' =>  Some(Token::RightAngle),    '<' =>  Some(Token::LeftAngle),
            '-' =>  Some(Token::Hyphen),        ',' =>  Some(Token::Comma),
            ':' =>  Some(Token::Colon),         '*' =>  Some(Token::Asterix),
            '+' =>  Some(Token::Plus),          '/' =>  Some(Token::ForwardSlash),
            '\\' => Some(Token::BackSlash),     '\"' => Some(Token::QoutationMark),
            ' ' =>  Some(Token::Whitespace),    '\n' => Some(Token::NewLine),
            '\r' => Some(Token::CarraigeReturn),
            _ => None
        }
    }

}



#[cfg(test)]
mod tests {
    use super::TokenParser;
    use super::super::lexemeparser::Parser;
    use super::super::super::slidingwindow::SlidingWindow;
    use super::super::super::token::Token;
   
    fn parser_helper(source: &str) -> Token {
        let parser = TokenParser;
        let mut phrase = SlidingWindow::new(source);
        parser.parse(&mut phrase)
    }

    #[test]
    fn test_parser_singletokens() {
        assert_eq!(parser_helper("+"), Token::Plus);
        assert_eq!(parser_helper("-"), Token::Hyphen);
        assert_eq!(parser_helper("*"), Token::Asterix);
        assert_eq!(parser_helper("{"), Token::OpenBrace);
        assert_eq!(parser_helper("}"), Token::CloseBrace);
        assert_eq!(parser_helper("\\"), Token::BackSlash);
        assert_eq!(parser_helper("\""), Token::QoutationMark);
        assert_eq!(parser_helper(" "), Token::Whitespace);
        assert_eq!(parser_helper("\r"), Token::CarraigeReturn);
        assert_eq!(parser_helper("\n"), Token::NewLine);
    }

    #[test]
    fn test_parser_compoundtokens() {
        assert_eq!(parser_helper("+="), Token::PlusEquals);
        assert_eq!(parser_helper("-="), Token::MinusEquals);
        assert_eq!(parser_helper("*="), Token::MultiplicationEquals);
        assert_eq!(parser_helper("/="), Token::DivideEquals);
        assert_eq!(parser_helper("=="), Token::ConditionalEquals);
        assert_eq!(parser_helper("++"), Token::IncrementOperator);
        assert_eq!(parser_helper("--"), Token::DecrementOperator);
        assert_eq!(parser_helper("->"), Token::PointerArrow);
    }

    #[test]
    #[should_panic]
    fn test_compoundtokensparser_invalidsequenceshouldfail() {
        let parser = TokenParser;
        parser.map_compound_token(Token::AsyncKeyword, Token::AwaitKeyword);
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