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
    let regex_exp = Regex::new("({|}|(|)|;)|\".+?\"|.+?(;| |(|)\\n)").unwrap();
    for captured_token in regex_exp.captures_iter(source_code) {
        println!("Token captured: {:?}", captured_token);
    }

    captured_tokens
}



