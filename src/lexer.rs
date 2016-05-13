use regex::Regex;

use syntax;


pub enum Token
{
    Operator,
    Identifier
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
                println!("Found a character: {}", character);
            }

            _ => slice_buffer.push(character)
        }
    }

    tokens
}



