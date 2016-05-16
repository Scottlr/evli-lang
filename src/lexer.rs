use syntax;

/*
pub fn tokenize(source_code: &str) -> Vec<syntax::Token> {
    let mut tokens = vec![];
    let mut slice_buffer = String::new();
 //   let mut characters = source_code.chars().collect();
   // let characters_len = characters.len();
    
    for mut character_index in 0..characters_len {
        match characters[character_index] {
            _ if syntax::OPERATORS.contains(&characters[character_index]) => {
                slice_buffer.clear();
                let current_token = syntax::map_operator_token(&characters[character_index].to_string());
                let next_character = characters.next().unwrap_or_default();
                if syntax::OPERATORS.contains(&next_character) {
                    let next_token = syntax::map_operator_token(&next_character.to_string());
                    let compound_token = syntax::map_compound_operator_token(current_token, next_token);
                }
                
                tokens.push(current_token);
                //println!("Found a character: {}", character);
            }
            _ => slice_buffer.push(characters[character_index])
        }
    }

    tokens
}

*/