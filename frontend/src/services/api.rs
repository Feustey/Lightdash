use crate::models::{SparkSeerStats, FeeHistory, PeerComparison, SuggestedPeer, SimulationResult};
use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen::JsValue;
use web_sys::window;

#[derive(Clone)]
pub struct ApiService {
    base_url: String,
    api_key: String,
    node_pubkey: String,
}

impl ApiService {
    pub fn new() -> Self {
        let window = window().expect("no global `window` exists");
        let env = window
            .get("env")
            .expect("no env object found")
            .dyn_into::<JsValue>()
            .expect("env is not an object");
        
        let base_url = js_sys::Reflect::get(&env, &"NEXT_PUBLIC_API_URL".into())
            .expect("NEXT_PUBLIC_API_URL not found")
            .as_string()
            .expect("NEXT_PUBLIC_API_URL is not a string");
            
        let api_key = js_sys::Reflect::get(&env, &"SPARKSEER_API_KEY".into())
            .expect("SPARKSEER_API_KEY not found")
            .as_string()
            .expect("SPARKSEER_API_KEY is not a string");
            
        let node_pubkey = js_sys::Reflect::get(&env, &"NODE_PUBKEY".into())
            .expect("NODE_PUBKEY not found")
            .as_string()
            .expect("NODE_PUBKEY is not a string");

        Self {
            base_url,
            api_key,
            node_pubkey,
        }
    }

    pub async fn get_node_stats(&self) -> Result<SparkSeerStats, String> {
        let url = format!("{}/v1/node/current-stats/{}", self.base_url, self.node_pubkey);
        
        let response = Request::get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: Value = response.json().await.map_err(|e| e.to_string())?;
        
        let stats = &json.as_array()
            .ok_or("Response is not an array")?[0];
            
        serde_json::from_value(stats.clone())
            .map_err(|e| e.to_string())
    }

    pub async fn get_fee_history(&self) -> Result<Vec<FeeHistory>, String> {
        let url = format!("{}/v1/node/fee-history/{}", self.base_url, self.node_pubkey);
        
        let response = Request::get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn get_peer_comparisons(&self) -> Result<Vec<PeerComparison>, String> {
        let url = format!("{}/v1/node/peer-comparisons/{}", self.base_url, self.node_pubkey);
        
        let response = Request::get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn get_suggested_peers(&self) -> Result<Vec<SuggestedPeer>, String> {
        let url = format!("{}/v1/node/suggested-peers/{}", self.base_url, self.node_pubkey);
        
        let response = Request::get(&url)
            .header("api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn simulate_fees(&self, base_fee: u64, fee_rate: u64) -> Result<SimulationResult, String> {
        let url = format!("{}/v1/services/simulate-fees/{}", self.base_url, self.node_pubkey);
        
        let params = serde_json::json!({
            "base_fee": base_fee,
            "fee_rate": fee_rate
        });

        let response = Request::post(&url)
            .header("api-key", &self.api_key)
            .json(&params)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        response.json().await.map_err(|e| e.to_string())
    }
} 