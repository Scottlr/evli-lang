pub struct TokenValidator;

impl TokenValidator {

	pub fn valid_numeral_char(character: char) -> bool {
        match character {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' 
                => true,
            _   => false        
        }
    }

    //Needs rewrite to calculate if character or not rather that 
    //matching on all characters. Will do for now.
    pub fn valid_alphabetical_character(character: char) -> bool {
        let lowered_phrase = character.to_lowercase().next().unwrap();
         match lowered_phrase {
             'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |
             'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |
             'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |
             'v' | 'w' | 'x' | 'y' | 'z' 
                => true,
            _   => false
        }
    } 
}