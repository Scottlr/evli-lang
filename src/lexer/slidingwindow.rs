use std::iter::FromIterator;


pub struct SlidingWindow {
    characters: Vec<char>,
    pub current_pos: usize,
    pub offset: usize,
    file_len: usize
}

impl SlidingWindow {
    pub fn new(source_code: &str) -> SlidingWindow {
     //   let chopped_source_code = source_code.chars().collect();
        SlidingWindow {
            characters: source_code.chars().collect(),
            current_pos: 0,
            offset: 0,
            file_len: source_code.len()
        }
    }

    pub fn peek(&self) -> char {
        self.characters[self.current_pos + 1].to_owned()
    }

    pub fn offset_peek(&self) -> char {
        self.characters[self.current_pos + self.offset].to_owned()
    }

    pub fn increase_offset(&mut self) {
        self.offset += 1;
    }
    pub fn current_character(&self) -> char {
        if self.characters[self.current_pos] == '+' {
            println!("Current token: {:?} | Next character: {}",
                self.characters[self.current_pos],
                self.characters[self.current_pos + 1]);
        }
        self.characters[self.current_pos].to_owned()
    }

    pub fn advance(&mut self) -> char {
        self.offset = 0;
        self.current_pos += 1;
        self.characters[self.current_pos].to_owned()
    }
    
    pub fn is_eof(&self) -> bool {
        self.current_pos >= (self.file_len - 1) || (self.current_pos + self.offset) >= (self.file_len - 1)
    }

    pub fn get_slice(&mut self) -> String {
        let slice = self.characters[self.current_pos .. (self.current_pos + self.offset)].to_owned();
        let converted_slice = String::from_iter(slice);
        self.current_pos += self.offset;
        self.offset = 0;
        converted_slice
    }

}