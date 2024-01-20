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
This is a summary of the two steps above: `Tokenizer` and `Filters`. Create a struct `Analyzer`:
```rs
pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}
```
Implement method `analyze` for `Analyzer`: takes a string slice, returns normalization tokens:
```rs
fn analyze(&self, text: &str) -> Vec<String> {
    let tokens = self.tokenizer.tokenize(text);
    let low = self.filters.lowercase(tokens);
    let stopped = self.filters.stop_words(low);
    self.filters.stemming(stopped).collect()
}
```
### 4. Indexing
Find the intersection of IDs for given tokens by Inverted Index technique.  
- Create an Inverted Index, using HashMap that has a String key and a HashSet value. The String represents a token and the HashSet holds the document IDs that contain that token.
    ```rs
    pub struct InvertedIndex {
        idx: HashMap<String, HashSet<u64>>,
        analyzer: Analyzer,
    }
    ``` 
- Run filters on the document and then adding document IDs to a set.
    - Input:
        ```
        [
            {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog",
            },
            {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer",
            },
        ]
        ```
    - Output:
        ```
        {
            "jump": {1},
            "quick": {1, 2},
            "summer": {2},
            "dog": {1, 2},
            "brown": {2, 1},
            "lazi": {1, 2},
            "leap": {2},
            "fox": {2, 1},
        }
        ```
    - Inplement:
        ```rs
        fn add(&mut self, docs: &[Document]) {
            for doc in docs {
                for token in self.analyzer.analyze(doc.text.as_str()) {
                    match self.idx.get_mut(&token) {
                        None => {
                            let v = HashSet::from([doc.id]);
                            self.idx.insert(token, v);
                        }
                        Some(v) => {
                            v.insert(doc.id);
                        }
                    }
                }
            }
        }
        ```
- Search text in index, identify the IDs that match the given tokens in index.  
    > Example: Where is text "brown foxes" ?

    - Tokens in Inverted Index data structure:
        ```
        {
            "jump": {1},
            "quick": {1, 2},
            "summer": {2},
            "dog": {1, 2},
            "brown": {2, 1},
            "lazi": {1, 2},
            "leap": {2},
            "fox": {2, 1},
        }
        ```
    - Result:
        ```
        {1, 2}
        ```
    - Implement:
        ```rs
        fn search(&self, text: &str) -> HashSet<u64> {
            let mut result: HashSet<u64> = HashSet::new();
            for token in self.analyzer.analyze(text) {
                match self.idx.get(&token) {
                    None => {}
                    Some(ids) => {
                        if result.is_empty() {
                            result = ids.clone();
                        }
                        result = result.intersection(ids).copied().collect()
                    }
                }
            }
            result
        }
        ```
## References
1. [https://early-etudes.com/posts/inverted-index/](https://early-etudes.com/posts/inverted-index/)