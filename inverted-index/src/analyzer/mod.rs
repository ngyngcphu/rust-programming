use crate::filters::{Filters, Language};
use crate::tokenizer::Tokenizer;

pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl Analyzer {
    pub fn new(language: Language) -> Self {
        Analyzer {
            tokenizer: Tokenizer::new(),
            filters: Filters::new(language),
        }
    }

    pub fn analyze(&self, text: &str) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text);
        let low = self.filters.lowercase(tokens);
        let stopped = self.filters.stop_words(low);
        self.filters.stemming(stopped).collect()
    }
}

#[cfg(test)]
mod analyzer_tests {
    use super::*;

    #[test]
    fn test_analyzer() {
        let analyzer = Analyzer::default();
        let text = "The rain, rain poured and poured, creating a rhythmic symphony of droplets on the windowpane!";
        let res = analyzer.analyze(text);
        // Result: ["rain", "pour", "creat", "rhythmic", "symphoni", "droplet", "windowpan"]
        assert_eq!(res.len(), 7, "text analyze failed");
    }
}
