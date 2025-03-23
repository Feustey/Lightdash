use crate::types::NodeStats;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub struct SparkseerService {
    api_key: String,
}

impl SparkseerService {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("SPARKSEER_API_KEY").unwrap_or_default(),
        }
    }

    pub async fn get_node_stats(&self) -> Result<NodeStats, String> {
        let url = "https://api.sparkseer.space/v1/node/stats";
        let request = Request::get(url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json");

        match request.send().await {
            Ok(response) => {
                if response.ok() {
                    match response.json::<NodeStats>().await {
                        Ok(stats) => Ok(stats),
                        Err(e) => Err(format!("Erreur lors du parsing des données : {}", e)),
                    }
                } else {
                    Err(format!("Erreur API : {}", response.status()))
                }
            }
            Err(e) => Err(format!("Erreur réseau : {}", e)),
        }
    }
} 