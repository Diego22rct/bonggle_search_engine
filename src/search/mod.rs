use std::collections::HashMap;

use regex::Regex;

use crate::network::Network;

#[derive(Debug)]
pub struct SearchEngine {
    index: HashMap<String, Vec<usize>>,
    documents: Vec<String>,
    network_queue: Network,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            documents: Vec::new(),
            network_queue: Network::new(),
        }
    }

    pub fn add_document(&mut self, document: String) {
        self.documents.push(document.clone());
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        if let Some(doc_ids) = self.index.get(&query.to_lowercase()) {
            doc_ids
                .iter()
                .map(|&id| self.documents[id].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Searches for a word within HTML tags in the documents.
    ///
    /// # Arguments
    ///
    /// * `word` - A string slice containing the word to search for within HTML tags.
    ///
    /// # Returns
    ///
    /// A vector of strings containing the HTML elements that match the word.
    pub fn search_in_html(&self, word: &str) -> Vec<String> {
        let re = Regex::new(&format!(r"(?i)<[^>]*>{}[^<]*</[^>]*>", word)).unwrap();
        let mut filtered_results = Vec::new();

        for document in &self.documents {
            for cap in re.captures_iter(document) {
                filtered_results.push(cap[0].to_string());
            }
        }

        filtered_results
    }

    pub fn index_document(&mut self, document: &str) {
        let id = self.documents.len();
        let words = document.split_whitespace();

        for word in words {
            let word = word.to_lowercase();
            let doc_ids = self.index.entry(word).or_insert(Vec::new());
            doc_ids.push(id);
        }
    }

    pub fn get_documents(&self) -> &Vec<String> {
        &self.documents
    }
}
