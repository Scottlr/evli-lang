use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lexer;
mod syntax;

fn main() {
    let file_contents = match read_file("lange.fir") {
        Ok(contents) => contents,
        Err(err) => panic!("failed reading file: {}", err)
    };
	
    lexer::tokenize(&file_contents);
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