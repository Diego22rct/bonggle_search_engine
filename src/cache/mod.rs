use std::collections::HashMap;

pub struct Cache {
    cache: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub async fn get_html(&mut self, url: &str) -> String {
        if let Some(html) = self.cache.get(url) {
            html.clone()
        } else {
            let html = reqwest::get(url).await.unwrap().text().await.unwrap();
            self.cache.insert(url.to_string(), html.clone());
            html
        }
    }
}
