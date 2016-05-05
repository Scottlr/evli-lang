use regex::Regex;

/*
#[derive(Debug)]
pub enum LanguageToken<'a>
{
	PrintStatement,
	SemiColon,
	String(&'a str),
	VariableDeclaration
}


pub fn tokenize<'a>(source_code: Vec<char>) -> Vec<Token<'a>> {
	let mut tokens = Vec::new(); 
	let mut token_buffer = String::new();

	for index in 0..(source_code.len() - 1) {
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
				let stringTokenValue = parse_string(&index, &source_code);
				tokens.push(Token::String { value : stringTokenValue });
				token_buffer = String::new();
			}
			_ => continue
		}
		
	}
	return tokens;
}
*/
pub fn tokenize_generic<'a>(source_code: &'a str)
{
	//let mut string_tokens = vec![];
	let regex_exp = Regex::new("({|}|(|)|;)|\".+?\"|.+?(;| |(|)\\n)").unwrap();

	for captured_token in regex_exp.captures_iter(source_code) {
	 
	    println!("Token captured: {:?}", captured_token);
	    
	}
	//string_tokens

}
