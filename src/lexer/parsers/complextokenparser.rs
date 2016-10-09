use super::super::slidingwindow::SlidingWindow;
use super::super::token:: { Token, TokenKind };
use super::lexemeparser::Parser;
use super::tokenvalidator::TokenValidator;

pub struct ComplexTokenParser;

impl Parser for ComplexTokenParser {
    fn parse(&self, src_code: &mut SlidingWindow) -> Token {
        let current_char = src_code.current_character();
        let token_kind = match current_char {
            '\"' => self.parse_string(src_code, false),
            '#' | _ if  TokenValidator::valid_numeral_char(src_code.offset_peek())  
                => self.parse_numerical_value(src_code),
            _  => self.parse_keyword_or_identifier(src_code)
        };
        Token::construct(token_kind, src_code)
    }

}

impl ComplexTokenParser {
    fn parse_keyword_or_identifier(&self, src_code: &mut SlidingWindow) -> TokenKind {
        let slice = src_code.conditional_slice(|src| src.can_offset_peek() 
            && TokenValidator::valid_char_sequence(src.offset_peek()));
        match self.map_keyword(&slice) {
            Some(value) => value,
            None        => TokenKind::Identifier(slice)
        }        
    }
    
    #[allow(unused_variables)]
    fn parse_string(&self, src_code: &mut SlidingWindow, string_literal: bool) -> TokenKind {
        src_code.advance();
        let slice = src_code.conditional_slice(|src| src.can_offset_peek() && src.offset_peek() != '\"' );
        TokenKind::StringValue(slice)
    }

    fn parse_numerical_value(&self, src_code: &mut SlidingWindow) -> TokenKind {
        let slice = src_code.conditional_slice(|src| src.can_offset_peek() && TokenValidator::valid_numeral_char(src.offset_peek()));
        TokenKind::NumericalValue(slice)
    }

    pub fn is_complex(&self, character: char) -> bool {
        TokenValidator::valid_alphabetical_character(character) || //Is a type/identifier/keyword
            TokenValidator::valid_numeral_char(character) ||    //Is numerical type
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
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::ComplexTokenParser;
    use super::super::lexemeparser::Parser;
    use super::super::super::slidingwindow::SlidingWindow;
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
        assert_eq!(parser_helper("02322 "), TokenKind::NumericalValue("02322".to_string()));
    }

}