use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct SearchEngine {
    index: HashMap<String, Vec<usize>>,
    documents: Vec<String>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            documents: Vec::new(),
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

    pub fn get_documents(&self) -> &Vec<String> {
        &self.documents
    }
}
