
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lexer;

#[cfg(not(test))]
fn main() {
    let file_contents = match read_file("lange.fir") {
        Ok(contents) => contents,
        Err(err) => panic!("failed reading file: {}", err)
    };
	
    println!("{}", file_contents);
    let lexer = lexer::Lexer;
    let tokens = lexer.tokenize(&file_contents);

    for token in tokens.clone() {
        println!("{:?}", token);
    }
    println!("Parsed tokens: {}", tokens.clone().len());
   
}


fn read_file<P: AsRef<Path>>(path: P) -> Result<String, String> {	
    File::open(path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents.to_string())
        })
}