use super::super::slidingwindow::SlidingWindow;
use super::super::token::Token;
use super::lexemeparser::Parser;

pub struct ComplexTokenParser;

impl Parser for ComplexTokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
        println!("Parsing a complex token...");
        let current_char = source_code.current_character();
        match current_char {
            '\"'    => self.parse_string(source_code, false),
            _       => self.parse_keyword_or_identifier(source_code)
        }
    }

}

impl ComplexTokenParser {

    pub fn is_complex(&self, current_character: char) -> bool {
        self.valid_character(current_character) || current_character == '\"'
    }
    //this might tie into parsing types
    fn parse_keyword_or_identifier(&self, source_code: &mut SlidingWindow) -> Token {
        while !source_code.is_eof() || !self.valid_character(source_code.peek()) {
            source_code.peek_offset_advance();
        }
        let phrase = source_code.get_slice();
        match self.map_keyword(&phrase) {
            Some(value) => value,
            None        => Token::Identifier(phrase)
        }
    }
    
    fn parse_string(&self, source_code: &mut SlidingWindow, string_literal: bool) -> Token {
        while !source_code.is_eof() || !self.valid_character(source_code.peek()) {
            source_code.advance_char();
        }
        Token::StringValue(source_code.get_slice())
    }

    fn map_keyword(&self, phrase: &str) -> Option<Token> {
        match phrase {
            "await" =>  Some(Token::AwaitKeyword),
            "func" =>   Some(Token::FuncKeyword),
            "pub" =>    Some(Token::PublicModifierKeyword),
            "int" =>    Some(Token::IntKeyword),
            "float" =>  Some(Token::FloatKeyword),
            "string" => Some(Token::StringKeyword),
            "for" =>    Some(Token::ForKeyword),
            "in" =>     Some(Token::InKeyword),
            "is" =>     Some(Token::IsKeyword),
            "where" =>  Some(Token::WhereKeyword),
            "loop" =>   Some(Token::LoopKeyword),
            "while" =>  Some(Token::WhileKeyword),
            "use" =>    Some(Token::UseKeyword),
            "class" =>  Some(Token::ClassKeyword),
            "struct" => Some(Token::StructKeyword),
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
             'v' | 'w' | 'x' | 'y' | 'z' | '-' | '_'
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
        assert_eq!(parser_helper("pub"), Token::PublicModifierKeyword);
    }

}