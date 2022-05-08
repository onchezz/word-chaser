mod logic;

pub use logic::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing_created_word_is_of_expected_type() {
        // let word_data: String = logic::functionality::get_file_data();
        let word: Word = logic::Word::new();

        let new_word = Word {
            word: String::from(""),
            meaning: String::from(""),
            example: String::from(""),
        };

        assert_eq!(&word.word, &new_word.word);
        assert_eq!(&word.meaning, &new_word.meaning);
        assert_eq!(&word.example, &new_word.example);
    }
    #[test]
    fn test() {}
}
