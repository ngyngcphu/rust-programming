# Rust-Programming
## Inverted Index
### 1. Tokenizer
1. Use crate [unicode_segmentation](unicode_segmentation) to split text into unicode words. If only filter words without punctuation:
    ```rs
    fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
        text.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect::<String>()
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into_iter()
    }
    ```
    it can lead to unintended errors, for example: `I'm -> Im`, which could cause confusion. Instead, change it to this:
    ```rs
    pub fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
        text.unicode_words()
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into_iter()
    }
    ```