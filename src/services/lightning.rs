use crate::models::lightning::{
    Channel, NodeInfo, Transaction,
    ChannelStatus, TransactionType, TransactionStatus,
    OutboundLiquidityValue
};
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Error;
use tracing::{instrument, info, error};

#[derive(Clone, PartialEq)]
pub struct LightningService {
    client: Client,
    base_url: String,
    sparkseer_url: String,
    ml_url: String,
}

impl LightningService {
    pub fn new(base_url: String, sparkseer_url: String, ml_url: String) -> Self {
        info!(base_url = %base_url, "Création du service Lightning");
        Self {
            client: Client::new(),
            base_url,
            sparkseer_url,
            ml_url,
        }
    }

    #[instrument(skip(self), err)]
    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        info!("Récupération des informations du nœud");
        let url = format!("{}/v1/getinfo", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                let info = response.json::<NodeInfo>().await?;
                info!(pubkey = %info.identity_pubkey, "Informations du nœud récupérées");
                Ok(info)
            }
            Err(e) => {
                error!(error = %e, "Erreur lors de la récupération des informations du nœud");
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self), err)]
    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        info!("Récupération de la liste des canaux");
        let url = format!("{}/v1/channels", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                let channels = response.json::<Vec<Channel>>().await?;
                info!(count = channels.len(), "Canaux récupérés");
                Ok(channels)
            }
            Err(e) => {
                error!(error = %e, "Erreur lors de la récupération des canaux");
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self), err)]
    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        info!("Récupération de la liste des transactions");
        let url = format!("{}/v1/transactions", self.base_url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                let transactions = response.json::<Vec<Transaction>>().await?;
                info!(count = transactions.len(), "Transactions récupérées");
                Ok(transactions)
            }
            Err(e) => {
                error!(error = %e, "Erreur lors de la récupération des transactions");
                Err(e.into())
            }
        }
    }

    #[instrument(skip(self), err)]
    pub async fn get_network_stats(&self) -> Result<Value> {
        info!("Récupération des statistiques réseau");
        let url = format!("{}/v1/network/info", self.sparkseer_url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                let stats = response.json::<Value>().await?;
                info!("Statistiques réseau récupérées");
                Ok(stats)
            }
            Err(e) => {
                error!(error = %e, "Erreur lors de la récupération des statistiques réseau");
                Err(e.into())
            }
        }
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

    #[instrument(skip(self), err)]
    pub async fn get_outbound_liquidity_value(&self) -> Result<OutboundLiquidityValue> {
        info!("Récupération des valeurs de liquidité sortante");
        let url = format!("{}/v1/liquidity/outbound", self.sparkseer_url);
        match self.client.get(&url).send().await {
            Ok(response) => {
                let value = response.json::<OutboundLiquidityValue>().await?;
                info!("Valeurs de liquidité sortante récupérées");
                Ok(value)
            }
            Err(e) => {
                error!(error = %e, "Erreur lors de la récupération des valeurs de liquidité sortante");
                Err(e.into())
            }
        }
    }
} 