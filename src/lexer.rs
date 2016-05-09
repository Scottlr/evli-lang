use regex::Regex;

pub enum Token<'a>
{
    String(&'a str)
}

fn phrase_to_token(captured_phrase: &str) -> Token {
    unimplemented!();
}

pub fn tokenize<'a>(source_code: &'a str) -> Vec<Token>
{
    let mut captured_tokens = vec![];
    println!("In tokenize method... attempting to create regex object");

    let regex_exp = match Regex::new("(\\*)|\".+?\"|.+?\\*") {
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };

 
    println!("Created object...");
    
    for captured_token in regex_exp.captures_iter(source_code) {
        println!("Token captured: {:?}", captured_token.at(0));
    }

    captured_tokens
}



