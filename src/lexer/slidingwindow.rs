use std::iter::FromIterator;
use super::token::TokenMetaData;

pub struct SlidingWindow {
    characters: Vec<char>,
    current_pos: usize,
    offset: usize,
    file_len: usize,
    current_line: usize,
    relative_line_pos: usize
}

impl SlidingWindow {
    pub fn new(source_code: &str) -> SlidingWindow {
        SlidingWindow {
            characters: source_code.chars().collect(),
            current_pos: 0,
            offset: 0,
            file_len: source_code.len(),
            current_line: 0,
            relative_line_pos: 0
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
    pub fn current_character(&mut self) -> char {
        let current_char = self.characters[self.current_pos].to_owned();
        if current_char == '\n' {
            self.current_line += 1;
            self.relative_line_pos = 0;
        }
        current_char
        
    }

    pub fn advance(&mut self)  {
        self.offset = 0;
        self.current_pos += 1;
    }
    
    pub fn is_eof(&self) -> bool {
        !self.can_peek()
    }

    pub fn can_peek(&self) -> bool {
        self.current_pos != self.file_len - 1
    }

    pub fn can_offset_peek(&self) -> bool {
        self.current_pos + self.offset != self.file_len - 1
    }

    pub fn get_slice(&mut self) -> String {
        let slice = self.characters[self.current_pos .. (self.current_pos + self.offset)].to_owned();
        let converted_slice = String::from_iter(slice);
        self.current_pos += self.offset;
        self.offset = 0;
        converted_slice
    }

    pub fn get_metadata(&self) -> TokenMetaData {
        TokenMetaData {
            parsed_on_line: self.current_line,
            relative_line_pos: self.relative_line_pos
        }
    }

    pub fn conditional_slice<F>(&mut self, condition: F) -> String
        where F : Fn(&mut SlidingWindow) -> bool
    {
        while condition(self)
        {
            self.increase_offset();
        }
        self.get_slice()
    }

}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

}