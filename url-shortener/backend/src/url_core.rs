use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
pub struct ShortUrl {
    pub key: String,
    pub original_url: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct UrlService {
    pub urls: HashMap<String, ShortUrl>,
}

impl UrlService {
    pub fn new() -> Self {
        Self {
            urls: HashMap::new(),
        }
    }

    pub fn get_short_url(&self, key: &str) -> Option<&ShortUrl> {
        self.urls.get(key)
    }

    pub fn create_short_url(&mut self, original_url: String) -> ShortUrl {
        if self.urls.contains_key(&original_url) {
            return self.urls.get(&original_url).unwrap().clone();
        }

        let hashed_code = Self::get_hashed_code(&original_url);

        let short_url = ShortUrl {
            key: hashed_code.clone(),
            original_url,
            created_at: chrono::Utc::now(),
        };

        self.urls.insert(hashed_code, short_url.clone());
        short_url
    }

    fn get_hashed_code(url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hash = hasher.finish();

        format!("{:016x}", hash)[..8].to_string()
    }
}
