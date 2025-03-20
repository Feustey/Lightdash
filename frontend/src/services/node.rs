use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use std::time::Instant;
use crate::{log_api_call, log_api_response, log_error, log_performance};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub num_channels: u32,
    pub active_channels: u32,
}

pub async fn search_node(query: &str) -> Result<Vec<NodeInfo>, String> {
    let start = Instant::now();
    
    // Si la requête ressemble à une pubkey (66 caractères), on fait une recherche directe
    if query.len() == 66 {
        log::debug!("Recherche directe par pubkey: {}", query);
        match get_node_info(query).await {
            Ok(node) => {
                log_performance!("recherche_node_pubkey", start.elapsed());
                Ok(vec![node])
            },
            Err(e) => {
                log_error!("Erreur lors de la recherche par pubkey", e);
                Ok(vec![])
            }
        }
    } else {
        // Sinon, on fait une recherche par alias
        let url = format!("https://1ml.com/node/search?q={}", query);
        log_api_call!("GET", &url);
        
        let result = Request::get(&url)
            .send()
            .await
            .map_err(|e| {
                log_error!("Erreur lors de la requête de recherche", e.to_string());
                e.to_string()
            })?;
            
        log_api_response!(result.status(), &url);
        
        let nodes = result.json().await.map_err(|e| {
            log_error!("Erreur lors du parsing de la réponse", e.to_string());
            e.to_string()
        })?;
        
        log_performance!("recherche_node_alias", start.elapsed());
        Ok(nodes)
    }
}

pub async fn get_node_info(pubkey: &str) -> Result<NodeInfo, String> {
    let start = Instant::now();
    let url = format!("https://1ml.com/node/{}/json", pubkey);
    log_api_call!("GET", &url);
    
    let result = Request::get(&url)
        .send()
        .await
        .map_err(|e| {
            log_error!("Erreur lors de la requête node info", e.to_string());
            e.to_string()
        })?;
        
    log_api_response!(result.status(), &url);
    
    let node = result.json().await.map_err(|e| {
        log_error!("Erreur lors du parsing des infos du nœud", e.to_string());
        e.to_string()
    })?;
    
    log_performance!("get_node_info", start.elapsed());
    Ok(node)
}

pub fn get_current_node() -> String {
    use gloo_storage::{LocalStorage, Storage};
    const DEFAULT_NODE: &str = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
    
    match LocalStorage::get("current_node_pubkey") {
        Ok(pubkey) => {
            log::debug!("Nœud courant récupéré du localStorage: {}", pubkey);
            pubkey
        },
        Err(_) => {
            log::info!("Utilisation du nœud par défaut: {}", DEFAULT_NODE);
            DEFAULT_NODE.to_string()
        }
    }
} 