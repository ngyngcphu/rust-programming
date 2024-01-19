use rust_stemmers::{Algorithm, Stemmer};
use stop_words;
use std::borrow::Cow;
use std::collections::HashSet;

pub enum Language {
    English,
}

pub struct Filters {
    stop_words_list: HashSet<String>,
    stemmer: Stemmer,
}

impl Language {
    fn get(&self) -> Algorithm {
        use self::Language::*;
        match self {
            English => Algorithm::English,
        }
    }

    fn get_stop_words(&self) -> HashSet<String> {
        use self::Language::*;
        let stop_words = match self {
            English => stop_words::get(stop_words::LANGUAGE::English),
        };
        stop_words.into_iter().collect::<HashSet<String>>()
    }
}

impl Default for Filters {
    fn default() -> Self {
        Filters::new(Language::English)
    }
}

impl Filters {
    fn new(language: Language) -> Self {
        Filters {
            stop_words_list: language.get_stop_words(),
            stemmer: Stemmer::create(language.get()),
        }
    }

    pub fn lowercase<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        tokens.map(|s| s.to_lowercase())
    }

    pub fn stop_words<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        let set_of_tokens: HashSet<String> = tokens.into_iter().collect();
        set_of_tokens
            .difference(&self.stop_words_list)
            .cloned()
            .collect::<Vec<String>>()
            .into_iter()
    }

    pub fn stemming<'a, I>(&'a self, tokens: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = String> + 'a,
    {
        tokens.map(|t| match self.stemmer.stem(&t) {
            Cow::Owned(stemmed_str) => stemmed_str,
            Cow::Borrowed(stemmed_str) => stemmed_str.to_string(),
        })
    }
}

#[cfg(test)]
mod filters_test {
    use super::*;

    #[test]
    fn test_lowercase() {
        let filter = Filters::default();
        let tokens = ["HELLO", "THIS", "IS", "PATRICK"]
            .into_iter()
            .map(str::to_string);
        let res: Vec<String> = filter.lowercase(tokens).collect();
        let expected = ["hello", "this", "is", "patrick"];
        assert_eq!(res, expected, "lowering case failed");
    }

    #[test]
    fn test_stop_words_default() {
        let filter = Filters::default();
        let tokens = ["as", "stay", "a", "will"].into_iter().map(str::to_string);
        let res: Vec<String> = filter.stop_words(tokens).collect();
        let expected = ["stay"];
        assert_eq!(res, expected, "stop words failed")
    }

    #[test]
    fn test_stemming_default() {
        let filter = Filters::default();
        let tokens = ["worked", "working", "works", "worker"]
            .into_iter()
            .map(str::to_string);
        let res: Vec<String> = filter.stemming(tokens).collect();
        let expected = ["work", "worker"];
        assert_eq!(res, expected, "stemming failed");
    }
}
