use syntax;


pub fn tokenize(source_code: &str) -> Vec<syntax::Token> {
    let mut tokens = vec![];
    let mut slice_buffer = String::new();

    for character in source_code.chars() {
        match character {
            _ if syntax::OPERATORS.contains(&character) => {
                slice_buffer.clear();
                tokens.push(syntax::map_operator_token(&character.to_string()));
                println!("Found a character: {}", character);
            }
            _ => slice_buffer.push(character)
        }
    }

    tokens
}