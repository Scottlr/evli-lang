
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

	for currentChar in source_code {
		if currentChar == '\"' {
			token_buffer = "\"".to_string();
		}
		else {
		    token_buffer = token_buffer + &(currentChar.to_string());
		}
		
		match token_buffer.as_ref() {
			"print" => {	
				tokens.push(Token::PrintStatement);
				token_buffer = String::new();
			},
			"\"" => {
				token_buffer =
			}
			_ => panic!("I dont know what it is?")
		}
		
	}


	return tokens;

}

