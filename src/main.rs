
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lexer;

fn main() {
    let file_contents = match read_file("lange.fir") {
        Ok(contents) => contents,
        Err(err) => panic!("failed reading file: {}", err)
    };
	
    println!("{}", file_contents);
    let lexer = lexer::Lexer::new();
    let tokens = lexer.tokenize(&file_contents);

    for token in tokens.clone() {
        println!("{:?}", token);
    }
    
    println!("Parsed tokens: {}", tokens.clone().len());
   
}

#[allow(dead_code)]
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, String> {	
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|file| read_file_contents(file))
}

fn read_file_contents(mut file: File) -> Result<String, String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| err.to_string())
        .map(|_| contents.to_string())
}