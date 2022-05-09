mod logic;

pub use logic::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing_created_word_is_of_expected_type() {
        // let word_data: String = logic::functionality::get_file_data();

        let new_word = Word {
            word: String::from("hello"),
            meaning: String::from("popular greetings"),
            example: String::from("hello there how are you doing"),
        };
        assert!(new_word.word.contains("hello"));
        assert!(new_word.meaning.contains("popular greetings"));
        assert!(new_word.example.contains("hello there how are you doing"));
    }
    #[test]
    fn test() {}
}
