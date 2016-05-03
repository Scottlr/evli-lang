
#[derive(Debug)]
pub enum Token
{
	PrintStatement,
	SemiColon,
	String {value: String },
	VariableDeclaration
}

pub fn tokenize(source_code: Vec<char>) -> Vec<Token> {
	let mut tokens = Vec::new(); 
	let mut token_buffer = String::new();

	for mut index in 0..(source_code.len() - 1) {
		if source_code[index] == '\"' {
			token_buffer = "\"".to_string();
		}
		else {
		    token_buffer = token_buffer + &source_code[index].to_string();
		}
		
		match token_buffer.as_ref() {
			"print" => {	
				tokens.push(Token::PrintStatement);
				token_buffer = String::new();
			},
			"\"" => {
				let stringTokenValue = parse_string(index, &source_code);
				tokens.push(Token::String { value : stringTokenValue });
				token_buffer = String::new();
			}
			_ => continue
		}
		
	}
	return tokens;
}

fn parse_string<'a>(current_index: &'a usize, source_code: &'a Vec<char>) -> String {
	let mut token_buffer = String::new();
	loop {
	    current_index = current_index + 1;
	    token_buffer += source_code[current_index];
	    if source_code[current_index] == '\"' && source_code[current_index - 1] != '\\' {
    		return token_buffer;
	    }
	}
}