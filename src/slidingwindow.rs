struct SlidingWindow {
    characters: Vec<char>;
    current_pos: i32;
    offset: i32;
    file_len: i32;
}

impl SlidingWindow {
    pub fn new(&str source_code) -> SlidingWindow {
        characters: source_code.chars().collect()
    }

    fn peek(&self) -> char {
        offset += 1;
        characters[current_pos + offset].to_owned();
    }

    fn advance_char(&self) {
        offset = 0;
        current_pos += 1;
    }
    

}