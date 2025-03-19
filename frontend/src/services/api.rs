use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use crate::models::{SparkSeerStats, FeeHistory, PeerComparison, SuggestedPeer, SimulationResult, Recommendation};
use serde_json::Value;
use serde_json::json;

#[derive(Clone, PartialEq)]
pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_node_health(&self) -> Result<SparkSeerStats, JsValue> {
        let url = format!("{}/api/node/health", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        response.json::<SparkSeerStats>().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn get_ai_recommendations(&self) -> Result<Vec<Recommendation>, JsValue> {
        let url = format!("{}/api/recommendations", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        response.json::<Vec<Recommendation>>().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn get_fee_history(&self) -> Result<FeeHistory, JsValue> {
        let url = format!("{}/api/fees/history", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        response.json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn get_peer_comparisons(&self) -> Result<Vec<PeerComparison>, JsValue> {
        let url = format!("{}/api/peers/compare", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        response.json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn get_suggested_peers(&self) -> Result<Vec<SuggestedPeer>, JsValue> {
        let url = format!("{}/api/peers/suggest", self.base_url);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
            
        response.json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn simulate_fees(&self, base_fee: u64, fee_rate: f64) -> Result<SimulationResult, JsValue> {
        let url = format!("{}/api/fees/simulate", self.base_url);
        let response = Request::post(&url)
            .json(&serde_json::json!({
                "base_fee": base_fee,
                "fee_rate": fee_rate
            }))
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        response.json::<SimulationResult>().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }
} 