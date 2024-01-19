# Rust-Programming
## Inverted Index
### 1. Tokenizer
Example:
- Original text: 
    ```
    "HellO #{$}! I'm 32-test suite.& IN NLP, ... TOKENIZATION IS A KEY STEP. IT BREAKS DOWN TEXT INTO WORDS OR TOKENS, PREPARING IT FOR ANALYSIS"
    ```
- Tokens:
    ```
    ["HellO", "I'm", "32", "test", "suite", "IN", "NLP", "TOKENIZATION", "IS", "A", "KEY", "STEP", "IT", "BREAKS", "DOWN", "TEXT", "INTO", "WORDS", "OR", "TOKENS", "PREPARING", "IT", "FOR", "ANALYSIS"]
    ```
If only filter words without punctuation like this:  
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
It can lead to unintended errors, for example: `I'M -> IM`, which could cause confusion. Instead of, use method `unicode_words` in crate [unicode_segmentation](https://github.com/unicode-rs/unicode-segmentation) to split text into unicode words. :

```rs
pub fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
    text.unicode_words()
        .map(str::to_string)
        .collect::<Vec<String>>()
        .into_iter()
}
```
### 2. Filter
Normalize tokens using techniques:
- Make all tokens lowercase
    - Original tokens:
        ```
        ["HellO", "I'm", "32", "test", "suite", "IN", "NLP", "TOKENIZATION", "IS", "A", "KEY", "STEP", "IT", "BREAKS", "DOWN", "TEXT", "INTO", "WORDS", "OR", "TOKENS", "PREPARING", "IT", "FOR", "ANALYSIS"]
        ```
    - Lowercase tokens:
        ```
        ["hello", "i'm", "32", "test", "suite", "in", "nlp", "tokenization", "is", "a", "key", "step", "it", "breaks", "down", "text", "into", "words", "or", "tokens", "preparing", "it", "for", "analysis"]
        ```
- Removes stop words from tokens ([Stopwords ISO](https://github.com/stopwords-iso/stopwords-iso))
    - Input:
        ```
        ["hello", "i'm", "32", "test", "suite", "in", "nlp", "tokenization", "is", "a", "key", "step", "it", "breaks", "down", "text", "into", "words", "or", "tokens", "preparing", "it", "for", "analysis"]
        ```
    - Output:
        ```
        ["suite", "step", "breaks", "analysis", "tokenization", "32", "preparing", "nlp", "tokens", "key"]
        ```
- Apply stemming technique ([Snowball compiler and stemming algorithms](https://github.com/snowballstem/snowball)) to all tokens
    - Input:
        ```
        ["suite", "step", "breaks", "analysis", "tokenization", "32", "preparing", "nlp", "tokens", "key"]
        ```
    - Output:
        ```
        ["suit", "step", "break", "analysi", "token", "32", "prepar", "nlp", "token", "key"]
        ```
Unfortunately, Snowball does not yet support Vietnamese.
### 3. Analyzer
This is a summary of the two steps above: `Tokenizer` and `Filter`. Create a struct `Analyzer`:
```rs
pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}
```
Implement method `analyze` for `Analyzer`: takes a string slice, return normalization tokens:
```rs
fn analyze(&self, text: &str) -> Vec<String> {
    let tokens = self.tokenizer.tokenize(text);
    let low = self.filters.lowercase(tokens);
    let stopped = self.filters.stop_words(low);
    self.filters.stemming(stopped).collect()
}
```