use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbossNode {
    pub pubkey: String,
    pub alias: Option<String>,
    pub capacity: i64,
    pub channel_count: i32,
    pub first_seen: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbossChannel {
    pub id: String,
    pub capacity: i64,
    pub node1_pubkey: String,
    pub node2_pubkey: String,
    pub last_update: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmbossResponse<T> {
    pub data: T,
}

pub struct AmbossService {
    client: Client,
    api_key: String,
}

impl AmbossService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get_node_info(&self, pubkey: &str) -> Result<AmbossNode> {
        let query = r#"
        query GetNodeInfo($pubkey: String!) {
            getNode(pubkey: $pubkey) {
                pubkey
                alias
                capacity
                channel_count
                first_seen
                updated_at
            }
        }
        "#;

        let variables = serde_json::json!({
            "pubkey": pubkey
        });

        let response = self.client
            .post("https://api.amboss.space/graphql")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await?;

        let data: AmbossResponse<serde_json::Value> = response.json().await?;
        
        // Extraction des données du nœud
        let node_data = data.data["getNode"].as_object().ok_or_else(|| anyhow::anyhow!("Node data not found"))?;
        
        Ok(AmbossNode {
            pubkey: node_data["pubkey"].as_str().unwrap_or_default().to_string(),
            alias: node_data["alias"].as_str().map(|s| s.to_string()),
            capacity: node_data["capacity"].as_i64().unwrap_or(0),
            channel_count: node_data["channel_count"].as_i64().unwrap_or(0) as i32,
            first_seen: node_data["first_seen"].as_str().unwrap_or_default().to_string(),
            updated_at: node_data["updated_at"].as_str().unwrap_or_default().to_string(),
        })
    }

    pub async fn get_node_channels(&self, pubkey: &str) -> Result<Vec<AmbossChannel>> {
        let query = r#"
        query GetNodeChannels($pubkey: String!) {
            getNodeChannels(pubkey: $pubkey) {
                id
                capacity
                node1_pubkey
                node2_pubkey
                last_update
                status
            }
        }
        "#;

        let variables = serde_json::json!({
            "pubkey": pubkey
        });

        let response = self.client
            .post("https://api.amboss.space/graphql")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await?;

        let data: AmbossResponse<serde_json::Value> = response.json().await?;
        
        let channels = data.data["getNodeChannels"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Channels data not found"))?;

        Ok(channels.iter().filter_map(|channel| {
            Some(AmbossChannel {
                id: channel["id"].as_str()?.to_string(),
                capacity: channel["capacity"].as_i64()?,
                node1_pubkey: channel["node1_pubkey"].as_str()?.to_string(),
                node2_pubkey: channel["node2_pubkey"].as_str()?.to_string(),
                last_update: channel["last_update"].as_str()?.to_string(),
                status: channel["status"].as_str()?.to_string(),
            })
        }).collect())
    }
} 