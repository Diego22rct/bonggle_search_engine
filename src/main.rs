use search_engine::{cache::Cache, network::Network, search::SearchEngine};

#[tokio::main]
async fn main() {
    let mut cache = Cache::new();
    let html = cache
        .get_html("https://es.wikipedia.org/wiki/Rust_(lenguaje_de_programaci%C3%B3n)")
        .await;

    let mut search_engine = SearchEngine::new();
    search_engine.add_document(html);

    let words_finded = search_engine.search_in_html("Rust");

    println!("Words finded: {:?}", words_finded);
}
