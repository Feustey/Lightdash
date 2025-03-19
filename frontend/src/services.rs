use crate::models::{Channel, NodeStats, Recommendation, Transaction};
use gloo_net::http::Request;
use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq)]
pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:3000/api".to_string(),
        }
    }

    pub async fn get_node_stats(&self) -> Result<NodeStats, JsValue> {
        let response = Request::get(&format!("{}/node/stats", self.base_url))
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let stats = response
            .json()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(stats)
    }

    pub async fn get_channels(&self) -> Result<Vec<Channel>, JsValue> {
        let response = Request::get(&format!("{}/channels", self.base_url))
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let channels = response
            .json()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(channels)
    }

    pub async fn get_transactions(&self) -> Result<Vec<Transaction>, JsValue> {
        let response = Request::get(&format!("{}/transactions", self.base_url))
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let transactions = response
            .json()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(transactions)
    }

    pub async fn get_recommendations(&self) -> Result<Vec<Recommendation>, JsValue> {
        let response = Request::get(&format!("{}/recommendations", self.base_url))
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let recommendations = response
            .json()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(recommendations)
    }
} 