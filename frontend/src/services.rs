use crate::models::{NodeInfo, Channel, Transaction, McpChannel, McpOffer};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use std::rc::Rc;
use serde_json::json;
use gloo_net::http::Request;
use crate::components::actions::{NodeHealth, ActionRecommendation};

const API_URL: &str = "http://localhost:3000/api";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const SPARKSEER_API_URL: &str = "https://api.sparkseer.space/v1";

pub struct ApiService {
    client: Rc<Client>,
    base_url: String,
    api_key: String,
    sparkseer_api_key: String,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            client: Rc::new(Client::new()),
            base_url: "http://localhost:3000".to_string(),
            api_key: dotenv::var("OPENAI_API_KEY").unwrap_or_default(),
            sparkseer_api_key: dotenv::var("SPARKSEER_API_KEY").unwrap_or_default(),
        }
    }

    pub fn get_node_info(&self, callback: Callback<Result<NodeInfo, String>>) {
        let client = self.client.clone();
        let url = format!("{}/api/node/info", self.base_url);
        
        spawn_local(async move {
            match client.get(&url).send().await {
                Ok(response) => {
                    match response.json::<NodeInfo>().await {
                        Ok(info) => callback.emit(Ok(info)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_channels(&self, callback: Callback<Result<Vec<Channel>, String>>) {
        let client = self.client.clone();
        let url = format!("{}/api/channels", self.base_url);
        
        spawn_local(async move {
            match client.get(&url).send().await {
                Ok(response) => {
                    match response.json::<Vec<Channel>>().await {
                        Ok(channels) => callback.emit(Ok(channels)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_transactions(&self, callback: Callback<Result<Vec<Transaction>, String>>) {
        let client = self.client.clone();
        let url = format!("{}/api/transactions", self.base_url);
        
        spawn_local(async move {
            match client.get(&url).send().await {
                Ok(response) => {
                    match response.json::<Vec<Transaction>>().await {
                        Ok(transactions) => callback.emit(Ok(transactions)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_mcp_channels(&self, callback: Callback<Result<Vec<McpChannel>, String>>) {
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("{}/mcp/channels", API_URL))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<Vec<McpChannel>>().await {
                        Ok(channels) => callback.emit(Ok(channels)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_mcp_offers(&self, callback: Callback<Result<Vec<McpOffer>, String>>) {
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("{}/mcp/offers", API_URL))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<Vec<McpOffer>>().await {
                        Ok(offers) => callback.emit(Ok(offers)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_node_health(&self, callback: Callback<Result<NodeHealth, String>>) {
        let sparkseer_api_key = self.sparkseer_api_key.clone();
        let node_pubkey = dotenv::var("NODE_PUBKEY").unwrap_or_default();
        
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("{}/node/current-stats/{}", SPARKSEER_API_URL, node_pubkey))
                .header("api-key", &sparkseer_api_key)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<SparkSeerNodeStats>().await {
                        Ok(stats) => {
                            let health = NodeHealth {
                                total_capacity: stats.total_capacity,
                                active_channels: stats.num_channels,
                                online_peers: stats.num_channels,
                                inbound_liquidity: stats.effective_inbound_balance as u64,
                                outbound_liquidity: stats.effective_outbound_balance as u64,
                                fee_earnings: 0,
                                uptime_percentage: 100.0 - (stats.htlc_response_time_mean * 100.0),
                            };
                            callback.emit(Ok(health))
                        }
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_channel_recommendations(&self, callback: Callback<Result<Vec<SparkSeerChannelRecommendation>, String>>) {
        let sparkseer_api_key = self.sparkseer_api_key.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("{}/services/channel-recommendations", SPARKSEER_API_URL))
                .header("api-key", &sparkseer_api_key)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<Vec<SparkSeerChannelRecommendation>>().await {
                        Ok(recommendations) => callback.emit(Ok(recommendations)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_liquidity_value(&self, callback: Callback<Result<SparkSeerLiquidityValue, String>>) {
        let sparkseer_api_key = self.sparkseer_api_key.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get(&format!("{}/services/outbound-liquidity-value", SPARKSEER_API_URL))
                .header("api-key", &sparkseer_api_key)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<SparkSeerLiquidityValue>().await {
                        Ok(value) => callback.emit(Ok(value)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn get_ai_recommendations(&self, health: NodeHealth, callback: Callback<Result<Vec<ActionRecommendation>, String>>) {
        let api_key = self.api_key.clone();
        let sparkseer_api_key = self.sparkseer_api_key.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            let channel_recs = Request::get(&format!("{}/services/channel-recommendations", SPARKSEER_API_URL))
                .header("api-key", &sparkseer_api_key)
                .send()
                .await
                .and_then(|resp| resp.json::<Vec<SparkSeerChannelRecommendation>>().await)
                .unwrap_or_default();

            let liquidity_value = Request::get(&format!("{}/services/outbound-liquidity-value", SPARKSEER_API_URL))
                .header("api-key", &sparkseer_api_key)
                .send()
                .await
                .and_then(|resp| resp.json::<SparkSeerLiquidityValue>().await)
                .unwrap_or_default();

            let prompt = format!(
                "En tant qu'expert en nœuds Lightning Network, analysez les métriques suivantes et les recommandations Sparkseer pour fournir 3 recommandations d'actions prioritaires :
                
                Métriques du nœud :
                - Capacité totale : {} sats
                - Canaux actifs : {} sur {} pairs en ligne
                - Liquidité entrante : {} sats
                - Liquidité sortante : {} sats
                - Gains en frais : {} sats
                - Disponibilité : {}%

                Recommandations Sparkseer :
                - Nombre de recommandations de canaux : {}
                - Recommandation principale : capacité idéale de {} sats avec frais passifs de {} ppm
                - Score de flexibilité de liquidité moyen : {}

                Formatez chaque recommandation avec une priorité (Haute/Moyenne/Basse), une action spécifique, une description détaillée et l'impact attendu.
                Répondez en JSON avec le format suivant :
                [{{
                    \"priority\": \"Haute/Moyenne/Basse\",
                    \"action\": \"Action à entreprendre\",
                    \"description\": \"Description détaillée\",
                    \"impact\": \"Impact attendu\"
                }}]",
                health.total_capacity,
                health.active_channels,
                health.online_peers,
                health.inbound_liquidity,
                health.outbound_liquidity,
                health.fee_earnings,
                health.uptime_percentage,
                channel_recs.len(),
                channel_recs.first().map(|r| r.info[0].ideal_capacity).unwrap_or(0),
                channel_recs.first().map(|r| r.info[0].passive_fee_ppm).unwrap_or(0),
                liquidity_value.channel_peers.len()
            );

            let payload = json!({
                "model": "gpt-4",
                "messages": [{
                    "role": "system",
                    "content": "Vous êtes un expert en Lightning Network qui analyse la santé des nœuds et fournit des recommandations d'actions concrètes basées sur les données Sparkseer et les métriques du nœud."
                }, {
                    "role": "user",
                    "content": prompt
                }],
                "temperature": 0.7
            });

            let response = Request::post(OPENAI_API_URL)
                .header("Authorization", &format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&payload).unwrap())
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<serde_json::Value>().await {
                        Ok(data) => {
                            let content = data["choices"][0]["message"]["content"]
                                .as_str()
                                .unwrap_or_default();
                            
                            match serde_json::from_str::<Vec<ActionRecommendation>>(content) {
                                Ok(recommendations) => callback.emit(Ok(recommendations)),
                                Err(e) => callback.emit(Err(format!("Erreur de parsing : {}", e))),
                            }
                        }
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn create_mcp_offer(&self, 
        capacity: u64,
        lease_fee_base: u64,
        lease_fee_rate: u32,
        lease_duration: u32,
        min_uptime: u32,
        callback: Callback<Result<McpOffer, String>>) {
        let payload = json!({
            "capacity": capacity,
            "lease_fee_base": lease_fee_base,
            "lease_fee_rate": lease_fee_rate,
            "lease_duration": lease_duration,
            "min_uptime": min_uptime,
        });

        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::post(&format!("{}/mcp/offers", API_URL))
                .json(&payload)
                .unwrap()
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<McpOffer>().await {
                        Ok(offer) => callback.emit(Ok(offer)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }

    pub fn accept_mcp_offer(&self, offer_id: &str, callback: Callback<Result<McpChannel, String>>) {
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::post(&format!("{}/mcp/offers/{}/accept", API_URL, offer_id))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.json::<McpChannel>().await {
                        Ok(channel) => callback.emit(Ok(channel)),
                        Err(e) => callback.emit(Err(e.to_string())),
                    }
                }
                Err(e) => callback.emit(Err(e.to_string())),
            }
        });
    }
} 