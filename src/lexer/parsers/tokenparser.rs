use super::super::token::Token;
use super::super::slidingwindow::SlidingWindow;
use super::lexemeparser::Parser;


pub struct TokenParser;

impl Parser for TokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        let mut token = self.map_token(source_code.current_character());
        if !source_code.is_eof() {
            match token {
                Token::Plus | Token::Hyphen | 
                Token::ForwardSlash | Token::Asterix | Token::Equals => {
                    let next_character = source_code.advance_char();
                    let next_token = self.map_token(next_character);
                    if next_token == Token::Equals {
                        token = self.map_compound_token(token, next_token);
                    }
                }
                _ => {}
            }
        }   
        token
    } 
}

impl TokenParser {
    pub fn map_compound_token(&self, current_token: Token, next_token: Token) -> Token {
        match (current_token, next_token) {
            (Token::Plus, Token::Equals) =>         Token::PlusEquals,
            (Token::Hyphen, Token::Equals) =>       Token::MinusEquals,
            (Token::Asterix, Token::Equals) =>      Token::MultiplicationEquals,
            (Token::ForwardSlash, Token::Equals) => Token::DivideEquals,
            (Token::Equals, Token::Equals) =>       Token::ConditionalEquals,
            _ => panic!("Invalid combinations for compound token...")
        }
    }

    pub fn map_token(&self, character: char) -> Token {
         match character {
            ';' => Token::SemiColon,    '(' => Token::OpenParen,
            ')' => Token::CloseParen,   '=' => Token::Equals,
            '{' => Token::OpenBrace,    '}' => Token::CloseBrace,
            '>' => Token::RightAngle,   '<' => Token::LeftAngle,
            '-' => Token::Hyphen,       ',' => Token::Comma,
            ':' => Token::Colon,        '*' => Token::Asterix,
            '+' => Token::Plus,         '/' => Token::ForwardSlash,
            '\\' => Token::BackSlash,   '\"' => Token::QoutationMark,
            ' ' => Token::Whitespace,   '\n' => Token::NewLine,
            '\r' => Token::CarraigeReturn,
            _ => panic!("Error trying to parse character: {}", character)
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