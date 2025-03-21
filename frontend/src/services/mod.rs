use log::error;
use crate::types::{NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees, Channel, Action, ActionType, ActionStatus};
use crate::config::{SPARKSEER_API_URL, OPENAI_API_URL, get_api_key};
use gloo_net::http::{Request, RequestBuilder, Headers};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::future::join4;
use uuid;
use chrono;

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

pub async fn fetch_node_stats() -> Result<NodeStats, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/node/stats", SPARKSEER_API_URL))
        .headers(headers)
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

pub async fn fetch_outbound_liquidity_value() -> Result<OutboundLiquidityValue, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/liquidity/outbound", SPARKSEER_API_URL))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_suggested_fees() -> Result<SuggestedFees, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/fees/suggested", SPARKSEER_API_URL))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_channels() -> Result<Vec<Channel>, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/channels", SPARKSEER_API_URL))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn fetch_actions() -> Result<Vec<Action>, String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::get(&format!("{}/actions", SPARKSEER_API_URL))
        .headers(headers)
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
        "Analysez les données suivantes d'un nœud Lightning Network et suggérez des actions d'optimisation :\n\n\
        Statistiques du nœud :\n\
        - Nombre de canaux : {}\n\
        - Capacité totale : {} sats\n\
        - Balance totale locale : {} sats\n\
        - Balance totale distante : {} sats\n\
        - Taille moyenne des canaux : {} sats\n\
        - Uptime : {}%\n\n\
        Canaux :\n{}",
        stats.num_channels,
        stats.total_capacity,
        stats.total_local_balance,
        stats.total_remote_balance,
        stats.avg_channel_size,
        stats.uptime_percentage,
        channels
            .iter()
            .map(|c| format!(
                "- Canal {} avec {} :\n  Capacité : {} sats, Balance locale : {} sats, Active : {}\n",
                c.channel_id, c.remote_alias, c.capacity, c.local_balance, c.active
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
                    type_: ActionType::OpenChannel,
                    status: ActionStatus::Pending,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
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

    Ok(actions)
}

const API_BASE_URL: &str = "http://localhost:8080/api";

pub async fn create_action(action_type: ActionType) -> Result<Action, String> {
    let response = Request::post(&format!("{}/actions", API_BASE_URL))
        .json(&json!({
            "type": action_type,
        }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    let action: Action = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(action)
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

pub async fn execute_action(action_id: &str) -> Result<(), String> {
    let api_key = get_api_key("sparkseer").ok_or_else(|| "API key not found".to_string())?;
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", api_key));
    headers.append("Content-Type", "application/json");

    let response = Request::post(&format!("{}/actions/{}/execute", SPARKSEER_API_URL, action_id))
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error! status: {}", response.status()));
    }

    Ok(())
} 