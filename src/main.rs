use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct QueueQuery {
    queries: Vec<String>,
}

impl QueueQuery {
    fn new() -> Self {
        Self {
            queries: Vec::new(),
        }
    }

    fn push(&mut self, query: String) {
        self.queries.push(query);
    }

    fn pop(&mut self) -> Option<String> {
        self.queries.pop()
    }

    fn search(&self, word: &str) -> Vec<&String> {
        self.queries
            .iter()
            .filter(|query| query.contains(word))
            .collect()
    }
}

#[derive(Debug)]
struct Fetcher {
    cache: HashMap<String, String>,
}

impl Fetcher {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    async fn fetch(&mut self, url: &str) -> String {
        if let Some(html) = self.cache.get(url) {
            html.clone()
        } else {
            let html = reqwest::get(url).await.unwrap().text().await.unwrap();
            self.cache.insert(url.to_string(), html.clone());
            html
        }
    }
}

#[derive(Debug)]
struct SearchEngine {
    index: HashMap<String, Vec<usize>>,
    documents: Vec<String>,
}

impl SearchEngine {
    fn new() -> Self {
        Self {
            index: HashMap::new(),
            documents: Vec::new(),
        }
    }

    fn add_document(&mut self, document: String) {
        let doc_id = self.documents.len();
        self.documents.push(document.clone());

        for word in document.split_whitespace() {
            self.index
                .entry(word.to_lowercase())
                .or_insert_with(Vec::new)
                .push(doc_id);
        }
    }

    fn search(&self, query: &str) -> Vec<String> {
        if let Some(doc_ids) = self.index.get(&query.to_lowercase()) {
            doc_ids
                .iter()
                .map(|&id| self.documents[id].clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    fn search_in_html(&self, word: &str) -> Vec<String> {
        let re = Regex::new(&format!(r"(?i)<[^>]*>{}[^<]*</[^>]*>", word)).unwrap();
        let mut filtered_results = Vec::new();

        for document in &self.documents {
            for cap in re.captures_iter(document) {
                filtered_results.push(cap[0].to_string());
            }
        }

        filtered_results
    }
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut fetcher = Fetcher::new();
        let html = fetcher.fetch("https://www.rust-lang.org").await;

        let mut engine = SearchEngine::new();

        engine.add_document(html);

        let results = engine.search_in_html("Rust");

        println!("Resultados: {:?}", results);
    });
}
