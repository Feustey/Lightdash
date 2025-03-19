use crate::models::lightning::{
    Channel, NodeInfo, Transaction,
    ChannelStatus, TransactionType, TransactionStatus
};
use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Error;

#[derive(Clone)]
pub struct LightningService {
    client: Client,
    base_url: String,
    sparkseer_url: String,
    ml_url: String,
}

impl LightningService {
    pub fn new(base_url: String, sparkseer_url: String, ml_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            sparkseer_url,
            ml_url,
        }
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        // Récupérer les informations de base du nœud
        let local_info: Value = self.client
            .get(&format!("{}/v1/getinfo", self.base_url))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Erreur lors de la connexion au nœud: {}", e))?
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Erreur lors de la lecture des données du nœud: {}", e))?;

        // Log des informations reçues pour le débogage
        println!("Réponse du nœud: {:?}", local_info);

        // Récupérer les informations enrichies de Sparkseer
        let node_pubkey = local_info["identity_pubkey"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Le champ identity_pubkey est manquant ou invalide"))?;

        let sparkseer_info: Value = self.client
            .get(&format!("{}/v1/node/{}", self.sparkseer_url, node_pubkey))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Erreur lors de la connexion à Sparkseer: {}", e))?
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Erreur lors de la lecture des données Sparkseer: {}", e))?;

        // Log des informations Sparkseer pour le débogage
        println!("Réponse de Sparkseer: {:?}", sparkseer_info);

        // Combiner les informations
        Ok(NodeInfo {
            pubkey: node_pubkey.to_string(),
            alias: local_info["alias"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Le champ alias est manquant ou invalide"))?
                .to_string(),
            capacity: sparkseer_info["capacity"]
                .as_u64()
                .or_else(|| local_info["total_capacity"].as_u64())
                .unwrap_or(0),
            channels: sparkseer_info["channels"]
                .as_u64()
                .or_else(|| local_info["num_active_channels"].as_u64())
                .unwrap_or(0) as u32,
            version: local_info["version"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Le champ version est manquant ou invalide"))?
                .to_string(),
        })
    }

    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        // Récupérer les canaux locaux
        let local_channels: Value = self.client
            .get(&format!("{}/v1/channels", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        // Enrichir avec les données de Sparkseer
        let mut channels = Vec::new();
        if let Some(channel_list) = local_channels["channels"].as_array() {
            for channel in channel_list {
                let remote_pubkey = channel["remote_pubkey"].as_str().unwrap_or_default();
                
                // Récupérer les informations du pair depuis Sparkseer
                let _peer_info: Value = self.client
                    .get(&format!("{}/v1/node/{}", self.sparkseer_url, remote_pubkey))
                    .send()
                    .await?
                    .json()
                    .await?;

                channels.push(Channel {
                    id: channel["chan_id"].as_str().unwrap_or_default().to_string(),
                    capacity: channel["capacity"].as_u64().unwrap_or(0),
                    local_balance: channel["local_balance"].as_u64().unwrap_or(0),
                    remote_balance: channel["remote_balance"].as_u64().unwrap_or(0),
                    remote_pubkey: remote_pubkey.to_string(),
                    status: if channel["active"].as_bool().unwrap_or(false) {
                        ChannelStatus::Active
                    } else {
                        ChannelStatus::Inactive
                    },
                });
            }
        }

        Ok(channels)
    }

    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        // Récupérer les transactions locales
        let local_txs: Value = self.client
            .get(&format!("{}/v1/payments", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        let mut transactions = Vec::new();
        if let Some(tx_list) = local_txs["payments"].as_array() {
            for tx in tx_list {
                transactions.push(Transaction {
                    id: tx["payment_hash"].as_str().unwrap_or_default().to_string(),
                    amount: tx["value_sat"].as_u64().unwrap_or(0),
                    fee: tx["fee_sat"].as_u64().unwrap_or(0),
                    timestamp: tx["creation_date"].as_i64().unwrap_or(0),
                    type_: match tx["status"].as_str().unwrap_or("") {
                        "SUCCEEDED" => TransactionType::Payment,
                        _ => TransactionType::Payment,
                    },
                    status: match tx["status"].as_str().unwrap_or("") {
                        "SUCCEEDED" => TransactionStatus::Completed,
                        "FAILED" => TransactionStatus::Failed,
                        _ => TransactionStatus::Pending,
                    },
                    description: tx["description"].as_str().map(String::from),
                });
            }
        }

        Ok(transactions)
    }

    pub async fn get_network_stats(&self) -> Result<Value> {
        // Récupérer les statistiques du réseau depuis 1ML
        let stats = self.client
            .get(&format!("{}/api/v1/network/statistics", self.ml_url))
            .send()
            .await?
            .json()
            .await?;
        
        Ok(stats)
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