use super::super::slidingwindow::SlidingWindow;

pub struct KeywordSyntaxParser {
    slidingwindow: &SlidingWindow
}

impl KeywordSyntaxParser {

    fn is_valid_identifier_character(&self, phrase: &str) -> bool {
        let converted = phrase.char_at(0).to_digit(10);
        match converted {
            Some(_) => true,
            None => false
        }
    }
}