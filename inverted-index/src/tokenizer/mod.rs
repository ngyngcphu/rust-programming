use unicode_segmentation::UnicodeSegmentation;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
        text.unicode_words()
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "Hello #{$}! I'm test suite & I ... contain number 32!!";
        let tokenizer = Tokenizer::new();
        let res: Vec<String> = tokenizer.tokenize(text).collect();
        let expected = [
            "Hello", "I'm", "test", "suite", "I", "contain", "number", "32",
        ];
        assert_eq!(res, expected, "tokenization failed");
    }
}
