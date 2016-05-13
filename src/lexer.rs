use regex::Regex;

use syntax;


pub enum Token
{
    SemiColon,
    OpenParen,
    CloseParen,
    Equals
}

fn phrase_to_token(captured_phrase: &str) -> Token {
    unimplemented!();
}

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut slice_buffer = String::new();

    for character in source_code.chars() {
        match character {
            _ if syntax::OPERATORS.contains(&character) => {
                slice_buffer.clear();
                tokens.push(identify_syntax_token(&character.to_string()));
                println!("Found a character: {}", character);
            }
            _ => slice_buffer.push(character)
        }
    }

    tokens
}

fn identify_syntax_token(phrase: &str) -> Token {
    match phrase {
        ";" => Token::SemiColon,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        "=" => Token::Equals,
        _ => panic!("Didn't recognise phrase to parse to token...")
    }
} 
