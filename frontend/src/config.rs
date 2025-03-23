use lazy_static::lazy_static;
use std::collections::HashMap;
use gloo_storage::{LocalStorage, Storage};

pub const SPARKSEER_API_URL: &str = "https://api.sparkseer.space/v1";
pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";

lazy_static! {
    static ref API_KEYS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("sparkseer", "SPARKSEER_API_KEY");
        m.insert("openai", "OPENAI_API_KEY");
        m
    };
}

pub fn set_api_key(service: &str, key: &str) {
    let _ = LocalStorage::set(format!("{}_api_key", service), key);
}

pub fn get_api_key(service: &str) -> Option<String> {
    API_KEYS.get(service).and_then(|key| LocalStorage::get(key).ok())
}

pub fn clear_api_key(service: &str) {
    LocalStorage::delete(format!("{}_api_key", service));
}

#[derive(Debug)]
pub enum ApiError {
    NetworkError(String),
    StorageError(String),
    ValidationError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Erreur réseau : {}", msg),
            ApiError::StorageError(msg) => write!(f, "Erreur de stockage : {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Erreur de validation : {}", msg),
        }
    }
}

impl From<gloo_storage::errors::StorageError> for ApiError {
    fn from(error: gloo_storage::errors::StorageError) -> Self {
        ApiError::StorageError(error.to_string())
    }
}

impl From<gloo_net::Error> for ApiError {
    fn from(error: gloo_net::Error) -> Self {
        ApiError::NetworkError(error.to_string())
    }
} 