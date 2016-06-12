use std::iter::FromIterator;


pub struct SlidingWindow {
    characters: Vec<char>,
    current_pos: usize,
    offset: usize,
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


    pub fn offset_peek(&self) -> char {
        self.characters[self.current_pos + self.offset].to_owned()
    }

    pub fn increase_offset(&mut self) {
        self.offset += 1;
    }
    pub fn current_character(&self) -> char {
        self.characters[self.current_pos].to_owned()
    }

    pub fn advance_char(&mut self) -> char {
        self.offset = 0;
        self.current_pos += 1;
        self.characters[self.current_pos].to_owned()
    }
    
    pub fn is_eof(&self) -> bool {
        self.current_pos >= (self.file_len - 1) ||
        self.offset >= (self.file_len - 1)
    }

    //Needs rewrite to potentially return reference to slice instead of a newly assigned String?
    pub fn get_slice(&mut self) -> String {
        let slice = self.characters[self.current_pos .. (self.current_pos + self.offset)].to_owned();
        let converted_slice = String::from_iter(slice);
        self.current_pos += self.offset;
        self.offset = 0;
        converted_slice
    }

}