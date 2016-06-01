use super::super::slidingwindow::SlidingWindow;
use super::super::token::*;
use super::lexemeparser::Parser;

pub struct ComplexTokenParser;

//Review name
impl Parser for ComplexTokenParser {
    fn parse(&self, source_code: &mut SlidingWindow) -> Token {
      /*  let current_char = source_code.current_character();
        match current_char {
            '\"' => {
                //parse string
            },
            _ => {

            }
        }
      */
        Token::AwaitKeyword
       // while source_code.P
    }

}

impl ComplexTokenParser {

    fn parse_keyword_or_identifier(self,source_code: &mut SlidingWindow) -> Token {
        Token::AwaitKeyword
    }
    
    fn parse_string(self, source_code: &mut SlidingWindow, string_literal: bool) -> Token {
        Token::StringKeyword
    }

    fn map_keyword(self, phrase: &str) -> Option<Token> {
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