use super::super::slidingwindow::SlidingWindow;
use super::super::token::Token;
use super::lexemeparser::Parser;

pub struct ComplexTokenParser;

impl Parser for ComplexTokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        let current_char = source_code.current_character();
        match current_char {
            '\"'    => self.parse_string(source_code, false),
            _       => self.parse_keyword_or_identifier(source_code)
        }
    }

}

impl ComplexTokenParser {
    fn parse_keyword_or_identifier(&self, source_code: &mut SlidingWindow) -> Token {
        while !source_code.is_eof() && self.valid_character(source_code.offset_peek()) {
            println!("Value from offset_peek(): {}", source_code.offset_peek());
            source_code.increase_offset();
        }
        let phrase = source_code.get_slice();
        match self.map_keyword(&phrase) {
            Some(value) => value,
            None        => Token::Identifier(phrase)
        }
    }
    
    #[allow(unused_variables)]
    fn parse_string(&self, source_code: &mut SlidingWindow, string_literal: bool) -> Token {
        source_code.advance();
        while !source_code.is_eof()  && self.valid_string_sequence(source_code.offset_peek()) {
        
            source_code.increase_offset();
        }
        Token::StringValue(source_code.get_slice())
    }

    pub fn is_complex(&self, current_character: char) -> bool {
        self.valid_character(current_character) || current_character == '\"'
    }
    
    fn valid_string_sequence(&self, current_character: char) -> bool {
        current_character != '\"' 
            && current_character == ' '
            || self.valid_character(current_character)
    }

    fn map_keyword(&self, phrase: &str) -> Option<Token> {
        match phrase {
            "await" =>  Some(Token::AwaitKeyword),
            "func" =>   Some(Token::FuncKeyword),
            "pub" =>    Some(Token::PublicModifierKeyword),
            "i32" =>    Some(Token::IntKeyword),
            "float" =>  Some(Token::FloatKeyword),
            "string" => Some(Token::StringKeyword),
            "for" =>    Some(Token::ForKeyword),
            "in" =>     Some(Token::InKeyword),
            "is" =>     Some(Token::IsKeyword),
            "where" =>  Some(Token::WhereKeyword),
            "while" =>  Some(Token::WhileKeyword),
            "uses" =>   Some(Token::UseKeyword),
            "class" =>  Some(Token::ClassKeyword),
            "struct" => Some(Token::StructKeyword),
            "async" =>  Some(Token::AsyncKeyword),
            "let" =>    Some(Token::LetKeyword),
            _ => None
        }
    }


    //Needs rewrite to calculate if character or not rather that 
    //matching on all characters. Will do for now.
    fn valid_character(&self, phrase: char) -> bool {
        let lowered_phrase = phrase.to_lowercase().next().unwrap();
        match lowered_phrase {
             'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |
             'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |
             'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
             'v' | 'w' | 'x' | 'y' | 'z' | '-' | '_' |
             '0' | '1' | '2' | '3' | '4' | '5' | '6' |
             '7' | '8' | '9'
                => true,
            _   => false
        }
        
    }
}


#[cfg(test)]
mod tests {
    use super::ComplexTokenParser;
    use super::super::lexemeparser::Parser;
    use super::super::super::slidingwindow::SlidingWindow;
    use super::super::super::token::Token;

    fn parser_helper(source: &str) -> Token {
        let parser = ComplexTokenParser;
        let mut phrase = SlidingWindow::new(source);
        parser.parse(&mut phrase)
    }
    
    #[test]
    fn test_parser_keywords() {
        //fuck off by one errors, is_eof doesn't seem to like offsets... so a space is npueeded to terminate
        assert_eq!(parser_helper("pub "), Token::PublicModifierKeyword);
        assert_eq!(parser_helper("i32 "), Token::IntKeyword);
        assert_eq!(parser_helper("for "), Token::ForKeyword);
        assert_eq!(parser_helper("await "), Token::AwaitKeyword);
        assert_eq!(parser_helper("string "), Token::StringKeyword);
        assert_eq!(parser_helper("uses "), Token::UseKeyword);
        assert_eq!(parser_helper("while "), Token::WhileKeyword);
        assert_eq!(parser_helper("class "), Token::ClassKeyword);
        assert_eq!(parser_helper("where "), Token::WhereKeyword);
        assert_eq!(parser_helper("async "), Token::AsyncKeyword);
        assert_eq!(parser_helper("in "), Token::InKeyword);  
        assert_eq!(parser_helper("struct "), Token::StructKeyword);
    }

    #[test]
    fn test_parser_stringvalues() {
        assert_eq!(parser_helper("\"some string value\""), Token::StringValue("some string value".to_string()));
    }

}