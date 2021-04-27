pub struct TokenValidator;

impl TokenValidator {

	pub fn valid_numeral_char(character: char) -> bool {
        match character {
            '0'..='9'
                => true,
            _   => false        
        }
    }

    pub fn valid_alphabetical_character(character: char) -> bool {
         match character {
             'a'..='z' | 'A'..='Z' => true,
            _   => false
        }
    } 

     // Returns a boolean flag indicating whether or not the passed character is
    // a valid character allowed in types/identifiers/keywords
    pub fn valid_char_sequence(character: char) -> bool {
        TokenValidator::valid_alphabetical_character(character) || 
            TokenValidator::valid_numeral_char(character) ||  
            character == '_' || character == '-'
        }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::TokenValidator;

    #[test]
    fn validcharsequence_validSequence_returnsTrue() {
        let validCharacterSequence = 'a';
        let validCharacterSequenceNumerical = '2';
        assert_eq!(TokenValidator::valid_char_sequence(validCharacterSequence), true);
        assert_eq!(TokenValidator::valid_char_sequence(validCharacterSequenceNumerical), true);
    }

    #[test]
    fn validcharsequence_invalidSequence_returnsFalse() {
        let invalidCharacterSequence = '#';
        assert_eq!(TokenValidator::valid_char_sequence(invalidCharacterSequence), false);
    }
 
}
