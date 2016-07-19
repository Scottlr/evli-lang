use super::super::slidingwindow::SlidingWindow;
use super::super::token:: { Token, TokenKind };
use super::lexemeparser::Parser;

pub struct ComplexTokenParser;

impl Parser for ComplexTokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        let current_char = source_code.current_character();
        match current_char {
            '\"'    
                => self.parse_string(source_code, false),
            '#' | _ if  self.valid_numerical_character(source_code.offset_peek()) 
                => self.parse_numerical_value(source_code),
            _       
                => self.parse_keyword_or_identifier(source_code)
        }
    }

}

impl ComplexTokenParser {
    fn parse_keyword_or_identifier(&self, source_code: &mut SlidingWindow) -> Token {
        while source_code.can_offset_peek() && self.valid_keyword_identifier_character(source_code.offset_peek()) {
            println!("Value from offset_peek(): {}", source_code.offset_peek());
            source_code.increase_offset();
        }
        let phrase = source_code.get_slice();
        let tokenkind = match self.map_keyword(&phrase) {
            Some(value) => value,
            None        => TokenKind::Identifier(phrase)
        };

        Token::construct(tokenkind, source_code)
    }
    
    #[allow(unused_variables)]
    fn parse_string(&self, source_code: &mut SlidingWindow, string_literal: bool) -> Token {
        source_code.advance();
        self.slide_until(source_code,|source_)
        while source_code.can_offset_peek() && source_code.offset_peek() != '\"' {
            source_code.increase_offset();
        }
        let tokenkind = TokenKind::StringValue(source_code.get_slice());
        Token::construct(tokenkind, source_code)
    }

    fn parse_numerical_value(&self, src_code: &mut SlidingWindow) -> Token {
        self.slide_until(src_code, |src_code| src_code.can_offset_peek() && self.valid_numerical_character(src_code.offset_peek()));
        let tokenkind = TokenKind::NumericalValue(src_code.get_slice());
        Token::construct(tokenkind, src_code)
    }

    fn slide_until<F>(&self, source_code: &mut SlidingWindow, loop_condtion: F)  
        where F : Fn(&mut SlidingWindow) -> bool {
            while loop_condtion(source_code) {
                source_code.increase_offset();
            }
    }

    pub fn is_complex(&self, character: char) -> bool {
        self.valid_alphabetical_character(character) || //Is a type/identifier/keyword
        self.valid_numerical_character(character) ||    //Is numerical type
        character == '\"'                               //Is a string
    }
    
    fn map_keyword(&self, phrase: &str) -> Option<TokenKind> {
        match phrase {
            "await" =>  Some(TokenKind::AwaitKeyword),
            "func" =>   Some(TokenKind::FuncKeyword),
            "pub" =>    Some(TokenKind::PublicModifierKeyword),
            "i32" =>    Some(TokenKind::IntKeyword),
            "float" =>  Some(TokenKind::FloatKeyword),
            "string" => Some(TokenKind::StringKeyword),
            "for" =>    Some(TokenKind::ForKeyword),
            "in" =>     Some(TokenKind::InKeyword),
            "is" =>     Some(TokenKind::IsKeyword),
            "where" =>  Some(TokenKind::WhereKeyword),
            "while" =>  Some(TokenKind::WhileKeyword),
            "uses" =>   Some(TokenKind::UseKeyword),
            "class" =>  Some(TokenKind::ClassKeyword),
            "struct" => Some(TokenKind::StructKeyword),
            "async" =>  Some(TokenKind::AsyncKeyword),
            "let" =>    Some(TokenKind::LetKeyword),
            _ => None
        }
    }

    // Returns a boolean flag indicating whether or not the passed character is
    // a valid character allowed in types/identifiers/keywords
    fn valid_keyword_identifier_character(&self, character: char) -> bool {
        self.valid_alphabetical_character(character) || 
        self.valid_numerical_character(character) || 
        character == '_' || character == '-'
    }

    //Needs rewrite to calculate if character or not rather that 
    //matching on all characters. Will do for now.
    fn valid_alphabetical_character(&self, character: char) -> bool {
        let lowered_phrase = character.to_lowercase().next().unwrap();
         match lowered_phrase {
             'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |
             'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |
             'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
             'v' | 'w' | 'x' | 'y' | 'z' 
                => true,
            _   => false
        }
    }

    fn valid_numerical_character(&self, character: char) -> bool {
        match character {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' 
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
    use super::super::super::token::TokenKind;

    fn parser_helper(source: &str) -> TokenKind {
        let parser = ComplexTokenParser;
        let mut phrase = SlidingWindow::new(source);
        parser.parse(&mut phrase).kind
    }
    
    #[test]
    fn test_parser_keywords() {
        assert_eq!(parser_helper("pub "),       TokenKind::PublicModifierKeyword);
        assert_eq!(parser_helper("i32 "),       TokenKind::IntKeyword);
        assert_eq!(parser_helper("for "),       TokenKind::ForKeyword);
        assert_eq!(parser_helper("await "),     TokenKind::AwaitKeyword);
        assert_eq!(parser_helper("string "),    TokenKind::StringKeyword);
        assert_eq!(parser_helper("uses "),      TokenKind::UseKeyword);
        assert_eq!(parser_helper("while "),     TokenKind::WhileKeyword);
        assert_eq!(parser_helper("class "),     TokenKind::ClassKeyword);
        assert_eq!(parser_helper("where "),     TokenKind::WhereKeyword);
        assert_eq!(parser_helper("async "),     TokenKind::AsyncKeyword);
        assert_eq!(parser_helper("in "),        TokenKind::InKeyword);  
        assert_eq!(parser_helper("struct "),    TokenKind::StructKeyword);
    }

    #[test]
    fn test_parser_stringvalues() {
        assert_eq!(parser_helper("\"some string value\""), TokenKind::StringValue("some string value".to_string()));
    }

    #[test]
    fn test_parser_numericalvalues() {
        //assert_eq!(parser_helper("02322 "), TokenKind::NumericalValue("02322"));
    }

}