use search_engine::{cache::Cache, search::SearchEngine};

#[tokio::test]
async fn test_cache_get_html() {
    let mut cache = Cache::new();
    let url = "https://es.wikipedia.org/wiki/Rust_(lenguaje_de_programaci%C3%B3n)";
    let html = cache.get_html(url).await;
    assert!(html.contains("Rust"));
}

#[test]
fn test_search_engine_add_document() {
    let mut search_engine = SearchEngine::new();
    let document = "<html>Rust is a programming language</html>".to_string();
    search_engine.add_document(document.clone());
    assert_eq!(search_engine.get_documents().len(), 1);
    assert_eq!(search_engine.get_documents()[0], document);
}

#[test]
fn test_search_engine_search_in_html() {
    let mut search_engine = SearchEngine::new();
    let document = "<html>Rust is a programming language</html>".to_string();
    search_engine.add_document(document);
    let results = search_engine.search_in_html("Rust");
    assert_eq!(results.len(), 1);
    assert!(results[0].contains("Rust"));
}
