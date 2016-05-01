pub enum Token
{
	SemiColon,
	String,
	VariableDeclaration
}

pub fn tokenize(source_code: Vec<char>) -> Vec<Token>
{
	let mut tokens = Vec::new(); 
	let mut token_buffer = "";
	for c in source_code 
	{
		println!("{}", c);
	}


	return tokens;

}

