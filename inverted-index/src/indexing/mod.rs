use std::collections::{HashMap, HashSet};

use crate::analyzer::Analyzer;
use crate::filters::Language;

pub struct InvertedIndex {
    idx: HashMap<String, HashSet<u64>>,
    analyzer: Analyzer,
}

pub struct Document {
    id: u64,
    text: String,
}

impl Default for InvertedIndex {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl InvertedIndex {
    fn new(language: Language) -> Self {
        Self {
            idx: HashMap::new(),
            analyzer: Analyzer::new(language),
        }
    }

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
}

#[cfg(test)]
mod indexing_tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut idx = InvertedIndex::default();
        let doc = [
            Document {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog".to_string(),
            },
            Document {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer".to_string(),
            },
        ];
        idx.add(&doc);
        //Result of indexing: {"jump": {1}, "quick": {1, 2}, "summer": {2}, "dog": {1, 2}, "brown": {2, 1}, "lazi": {1, 2}, "leap": {2}, "fox": {2, 1}}
        assert_eq!(idx.idx.keys().len(), 8, "adding to index failed");
    }

    #[test]
    fn search_on_one_phrase_test() {
        let mut idx = InvertedIndex::default();
        let doc = [
            Document {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog".to_string(),
            },
            Document {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer".to_string(),
            },
        ];
        idx.add(&doc);
        let result = idx.search("dogs in summer");
        //Result: {2}
        assert_eq!(result.get(&2), Some(&2), "searching on one phrase failed");
    }

    #[test]
    fn intersection_search_text() {
        let mut idx = InvertedIndex::default();
        let doc = [
            Document {
                id: 1,
                text: "The quick brown fox jumped over the lazy dog".to_string(),
            },
            Document {
                id: 2,
                text: "Quick brown foxes leap over lazy dogs in summer".to_string(),
            },
        ];
        idx.add(&doc);
        let result = idx.search("brown foxes");
        assert_eq!(result, HashSet::from([1, 2]), "intersection search failed");
    }
}
