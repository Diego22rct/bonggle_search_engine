mod cache;
mod network;
mod search;

use cache::Cache;
use search::SearchEngine;

#[tokio::main]
async fn main() {
    let mut cache = Cache::new();
    let html = cache
        .get_html("https://es.wikipedia.org/wiki/Rust_(lenguaje_de_programaci%C3%B3n)")
        .await;
    //println!("HTML: {}", html);

    let mut search_engine = SearchEngine::new();
    search_engine.add_document(html);

    let mut words_finded = search_engine.search_in_html("Rust");

    println!("Words finded: {:?}", words_finded);
    // println!("{:?}", search_engine);
}
