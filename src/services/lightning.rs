use crate::models::lightning::{Channel, NodeInfo, Transaction};
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use anyhow::Error;

pub struct LightningService {
    client: Client,
    base_url: String,
}

impl LightningService {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        let response = self.client
            .get(&format!("{}/v1/getinfo", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        
        Ok(response)
    }

    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        let response = self.client
            .get(&format!("{}/v1/channels", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        
        Ok(response)
    }

    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let response = self.client
            .get(&format!("{}/v1/transactions", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        
        Ok(response)
    }

    pub async fn send_payment(&self, payment_request: &str) -> Result<Value, Error> {
        let response = self.client
            .post(format!("{}/v1/channels/transactions", self.base_url))
            .json(&json!({
                "payment_request": payment_request,
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn create_invoice(&self, amount: u64, description: &str) -> Result<Value, Error> {
        let response = self.client
            .post(format!("{}/v1/invoices", self.base_url))
            .json(&json!({
                "amount": amount,
                "description": description,
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn open_channel(&self, pubkey: &str, amount: u64) -> Result<Value, Error> {
        let response = self.client
            .post(format!("{}/v1/channels", self.base_url))
            .json(&json!({
                "node_pubkey": pubkey,
                "amount": amount,
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn close_channel(&self, channel_id: &str) -> Result<Value, Error> {
        let response = self.client
            .delete(format!("{}/v1/channels/{}", self.base_url, channel_id))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
} 