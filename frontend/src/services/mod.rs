use log::error;
use crate::types::{NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees, Channel, Action, ActionType, ActionStatus, ChannelStatus};
use crate::config::{SPARKSEER_API_URL, OPENAI_API_URL, get_api_key};
use gloo_net::http::{Request, RequestBuilder, Headers};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::future::join4;
use uuid;
use chrono;
use wasm_bindgen_futures::spawn_local;
use yew::callback::Callback;

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u32 = 1000;

async fn retry_request<T, F>(f: F) -> Result<T, String>
where
    T: 'static,
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, String>> + 'static>> + 'static,
{
    let mut retries = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    return Err(error);
                }
                error!("Request failed, retrying ({}/{}): {}", retries, MAX_RETRIES, error);
                gloo_timers::future::TimeoutFuture::new(RETRY_DELAY_MS * retries).await;
            }
        }
    }
}

fn create_request(url: &str, api_key: Option<String>) -> RequestBuilder {
    let request = Request::get(url);
        
    if let Some(key) = api_key {
        let headers = Headers::new();
        headers.append("Authorization", &format!("Bearer {}", key));
        request.headers(headers)
    } else {
        request
    }
}

pub type Error = String;

pub async fn fetch_node_stats() -> Result<NodeStats, Error> {
    let response = Request::get(&format!("{}/node/stats", SPARKSEER_API_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_channel_recommendations() -> Result<Vec<ChannelRecommendation>, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/recommendations", SPARKSEER_API_URL))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_outbound_liquidity_value() -> Result<OutboundLiquidityValue, Error> {
    let response = Request::get(&format!("{}/liquidity/outbound", SPARKSEER_API_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_suggested_fees() -> Result<SuggestedFees, Error> {
    let response = Request::get(&format!("{}/fees/suggested", SPARKSEER_API_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_channels() -> Result<Vec<Channel>, Error> {
    Ok(vec![
        Channel {
            channel_id: "123".to_string(),
            remote_pubkey: "abc".to_string(),
            remote_alias: "Node 1".to_string(),
            capacity: 1_000_000,
            local_balance: 500_000,
            remote_balance: 500_000,
            active: true,
            status: ChannelStatus::Active,
        }
    ])
}

pub async fn fetch_actions() -> Result<Vec<Action>, Error> {
    let response = Request::get(&format!("{}/actions", SPARKSEER_API_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_all_data() -> Result<(NodeStats, Vec<ChannelRecommendation>, OutboundLiquidityValue, SuggestedFees), String> {
    let (stats, recommendations, liquidity, fees) = join4(
        fetch_node_stats(),
        fetch_channel_recommendations(),
        fetch_outbound_liquidity_value(),
        fetch_suggested_fees(),
    )
    .await;

    Ok((stats?, recommendations?, liquidity?, fees?))
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

pub async fn get_ai_actions(node_stats: &NodeStats, channels: &[Channel]) -> Result<Vec<Action>, String> {
    let prompt = prepare_prompt(node_stats, channels);
    let messages = vec![
        OpenAIMessage {
            role: "system".to_string(),
            content: "You are a Lightning Network optimization assistant. Analyze node statistics and channel data to suggest concrete actions for improving node performance and routing efficiency.".to_string(),
        },
        OpenAIMessage {
            role: "user".to_string(),
            content: prompt,
        },
    ];

    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.7,
    };

    let api_key = get_api_key("openai").ok_or_else(|| "OpenAI API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::post(&format!("{}/chat/completions", OPENAI_API_URL))
        .headers(headers)
        .json(&request)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("OpenAI API request failed with status: {}", response.status()));
    }

    let response: OpenAIResponse = response.json().await.map_err(|e| e.to_string())?;
    let content = response.choices.first()
        .ok_or_else(|| "No response from OpenAI".to_string())?
        .message.content.clone();

    parse_ai_response(&content)
}

fn prepare_prompt(stats: &NodeStats, channels: &[Channel]) -> String {
    format!(
        "Analysez les données suivantes du nœud Lightning Network de Feustey (pubkey: 0296b2db342fcf87ea94d981757fdf4d3e545bd5cef4919f58b5d38dfdd73bf5c9) et suggérez des actions d'optimisation :\n\n\
        Statistiques du nœud :\n\
        - Pubkey : {}\n\
        - Alias : {}\n\
        - Nombre de canaux : {}\n\
        - Capacité totale : {} sats\n\
        - Balance totale locale : {} sats\n\
        - Balance totale distante : {} sats\n\
        - Taille moyenne des canaux : {} sats\n\
        - Uptime : {}%\n\n\
        Canaux :\n{}\n\n\
        Veuillez suggérer des actions concrètes pour optimiser ce nœud, en vous concentrant sur :\n\
        1. L'équilibrage des canaux\n\
        2. L'ouverture ou la fermeture de canaux\n\
        3. L'ajustement des frais\n\
        4. L'amélioration de la connectivité\n\
        Pour chaque suggestion, indiquez :\n\
        - Le type d'action (OpenChannel, CloseChannel, UpdateFees, Rebalance)\n\
        - La priorité (1-5, où 1 est la plus haute)\n\
        - L'impact attendu sur le nœud",
        stats.pubkey,
        stats.alias,
        stats.num_channels,
        stats.total_capacity,
        stats.local_balance,
        stats.remote_balance,
        stats.avg_channel_size,
        stats.uptime_percentage,
        channels
            .iter()
            .map(|c| format!(
                "- Canal {} avec {} :\n  Capacité : {} sats, Balance locale : {} sats ({:.1}%), Active : {}\n",
                c.channel_id,
                c.remote_pubkey,
                c.capacity,
                c.local_balance,
                (c.local_balance as f64 / c.capacity as f64) * 100.0,
                c.active
            ))
            .collect::<Vec<_>>()
            .join("")
    )
}

fn parse_ai_response(content: &str) -> Result<Vec<Action>, String> {
    let mut actions = Vec::new();
    let mut current_action = None;
    let mut description = String::new();
    let mut priority = None;
    let mut impact = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let (Some(action_type), Some(prio)) = (current_action.take(), priority.take()) {
                actions.push(Action {
                    id: uuid::Uuid::new_v4().to_string(),
                    status: ActionStatus::Pending,
                    action_type,
                    description: description.trim().to_string(),
                    priority: prio,
                    impact: impact.trim().to_string(),
                });
                description.clear();
                impact.clear();
            }
            continue;
        }

        if line.starts_with("Action:") || line.starts_with("Type:") {
            current_action = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
        } else if line.starts_with("Priority:") || line.starts_with("Priorité:") {
            priority = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
        } else if line.starts_with("Impact:") {
            impact = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else {
            description.push_str(line);
            description.push('\n');
        }
    }

    if actions.is_empty() {
        return Err("Aucune action n'a pu être extraite de la réponse".to_string());
    }

    actions
}

const API_BASE_URL: &str = "https://lightdash.netlify.app/api";

pub struct ApiService;

impl ApiService {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch_node_stats(&self, pubkey: &str, callback: Callback<Result<NodeStats, String>>) {
        let url = format!("{}/node/{}", API_BASE_URL, pubkey);
        let callback = callback.clone();

        spawn_local(async move {
            let response = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !response.ok() {
                return Err(format!("HTTP error! status: {}", response.status()));
            }

            let stats: NodeStats = response.json().await.map_err(|e| e.to_string())?;
            let _ = stats; ()
        });
    }

    pub fn fetch_channels(&self, pubkey: &str, callback: Callback<Result<Vec<Channel>, String>>) {
        let url = format!("{}/node/{}/channels", API_BASE_URL, pubkey);
        let callback = callback.clone();

        spawn_local(async move {
            let response = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !response.ok() {
                return Err(format!("HTTP error! status: {}", response.status()));
            }

            let channels: Vec<Channel> = response.json().await.map_err(|e| e.to_string())?;
            let _ = channels; ()
        });
    }

    pub fn fetch_actions(&self, callback: Callback<Result<Vec<Action>, String>>) {
        let url = format!("{}/actions", API_BASE_URL);
        let callback = callback.clone();

        spawn_local(async move {
            let response = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !response.ok() {
                return Err(format!("HTTP error! status: {}", response.status()));
            }

            let actions: Vec<Action> = response.json().await.map_err(|e| e.to_string())?;
            let _ = actions; ()
        });
    }

    pub fn create_action(&self, action_type: ActionType, callback: Callback<Result<Action, String>>) {
        let url = format!("{}/actions", API_BASE_URL);
        let callback = callback.clone();

        spawn_local(async move {
            let response = gloo_net::http::Request::post(&url)
                .json(&json!({
                    "type": action_type.to_string(),
                    "status": ActionStatus::Pending.to_string(),
                }))
                .map_err(|e| e.to_string())?
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !response.ok() {
                return Err(format!("HTTP error! status: {}", response.status()));
            }

            let action: Action = response.json().await.map_err(|e| e.to_string())?;
            let _ = action; ()
        });
    }

    pub fn get_recommendations(&self, pubkey: &str, callback: Callback<Result<Vec<ChannelRecommendation>, String>>) {
        let url = format!("{}/node/{}/recommendations", API_BASE_URL, pubkey);
        let callback = callback.clone();

        spawn_local(async move {
            let response = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !response.ok() {
                return Err(format!("HTTP error! status: {}", response.status()));
            }

            let recommendations: Vec<ChannelRecommendation> = response.json().await.map_err(|e| e.to_string())?;
            let _ = recommendations; ()
        });
    }
}

pub async fn cancel_action(action_id: &str) -> Result<(), String> {
    let response = Request::delete(&format!("{}/actions/{}", API_BASE_URL, action_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    Ok(())
}

pub async fn execute_action(action_type: ActionType, prio: u32, impact: &str) -> Result<Action, Error> {
    let payload = serde_json::json!({
        "type": action_type,
        "priority": prio,
        "impact": impact
    });

    let response = Request::post(&format!("{}/actions", SPARKSEER_API_URL))
        .json(&payload)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
} 