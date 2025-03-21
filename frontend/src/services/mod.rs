use gloo_net::http::{Request, RequestBuilder, Headers, Method};
use serde::{Deserialize, Serialize};
use crate::types::{NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees, Channel, Action};
use crate::config::{SPARKSEER_API_URL, OPENAI_API_URL, get_api_key};
use log::{info, error, debug};
use wasm_bindgen::prelude::*;
use serde_json::json;
use gloo_utils::format::JsValueSerdeExt;

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
        let mut headers = Headers::new();
        headers.append("Authorization", &format!("Bearer {}", key));
        request.headers(headers)
    } else {
        request
    }
}

pub async fn fetch_node_stats() -> Result<NodeStats, String> {
    debug!("Fetching node stats");

    retry_request(move || {
        let url = format!("{}/node/stats", SPARKSEER_API_URL);
        debug!("Fetching from {}", url);
        
        Box::pin(async move {
            let request = create_request(&url, get_api_key("sparkseer"));
            
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("Server responded with status: {}", response.status()));
            }

            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        })
    }).await
}

pub async fn fetch_channel_recommendations() -> Result<Vec<ChannelRecommendation>, String> {
    debug!("Fetching channel recommendations");

    retry_request(move || {
        let url = format!("{}/recommendations", SPARKSEER_API_URL);
        debug!("Fetching from {}", url);
        
        Box::pin(async move {
            let request = create_request(&url, get_api_key("sparkseer"));
            
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("Server responded with status: {}", response.status()));
            }

            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        })
    }).await
}

pub async fn fetch_outbound_liquidity_value() -> Result<OutboundLiquidityValue, String> {
    debug!("Fetching outbound liquidity value");

    retry_request(move || {
        let url = format!("{}/liquidity/outbound", SPARKSEER_API_URL);
        debug!("Fetching from {}", url);
        
        Box::pin(async move {
            let request = create_request(&url, get_api_key("sparkseer"));
            
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("Server responded with status: {}", response.status()));
            }

            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        })
    }).await
}

pub async fn fetch_suggested_fees() -> Result<SuggestedFees, String> {
    debug!("Fetching suggested fees");

    retry_request(move || {
        let url = format!("{}/fees/suggested", SPARKSEER_API_URL);
        debug!("Fetching from {}", url);
        
        Box::pin(async move {
            let request = create_request(&url, get_api_key("sparkseer"));
            
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("Server responded with status: {}", response.status()));
            }

            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        })
    }).await
}

pub async fn fetch_channels() -> Result<Vec<Channel>, String> {
    debug!("Fetching channels");

    retry_request(move || {
        let url = format!("{}/channels", SPARKSEER_API_URL);
        debug!("Fetching from {}", url);
        
        Box::pin(async move {
            let request = create_request(&url, get_api_key("sparkseer"));
            
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("Server responded with status: {}", response.status()));
            }

            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        })
    }).await
}

pub async fn fetch_all_data() -> Result<(NodeStats, Vec<ChannelRecommendation>, OutboundLiquidityValue, SuggestedFees), String> {
    let (node_stats, channel_recommendations, outbound_liquidity, suggested_fees) = futures::future::try_join4(
        fetch_node_stats(),
        fetch_channel_recommendations(),
        fetch_outbound_liquidity_value(),
        fetch_suggested_fees(),
    ).await?;

    Ok((node_stats, channel_recommendations, outbound_liquidity, suggested_fees))
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

pub async fn get_ai_actions(node_stats: &NodeStats, channels: &Vec<Channel>) -> Result<Vec<Action>, String> {
    debug!("Getting AI actions");

    let prompt = prepare_prompt(node_stats, channels);
    let messages = vec![
        json!({
            "role": "system",
            "content": "You are a Lightning Network optimization assistant. Analyze node statistics and channel data to suggest concrete actions for improving node performance and routing efficiency."
        }),
        json!({
            "role": "user",
            "content": prompt
        })
    ];

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "temperature": 0.7
    });

    retry_request(move || {
        let url = format!("{}/chat/completions", OPENAI_API_URL);
        let body = body.clone();
        debug!("Sending request to OpenAI API at {}", url);
        
        Box::pin(async move {
            let mut headers = Headers::new();
            headers.append("Content-Type", "application/json");
            headers.append("Authorization", &format!("Bearer {}", get_api_key("openai").unwrap_or_default()));

            let body_string = serde_json::to_string(&body)
                .map_err(|e| format!("Failed to serialize request body: {}", e))?;

            let request = Request::post(&url)
                .headers(headers)
                .body(body_string)
                .map_err(|e| format!("Failed to set request body: {}", e))?;

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => return Err(format!("Failed to send request: {}", e)),
            };

            if !response.ok() {
                return Err(format!("OpenAI API request failed with status: {}", response.status()));
            }

            let result = response.json::<serde_json::Value>().await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            let content = result["choices"][0]["message"]["content"]
                .as_str()
                .ok_or_else(|| "Invalid response format".to_string())?;

            parse_ai_response(content)
        })
    }).await
}

fn prepare_prompt(stats: &NodeStats, channels: &[Channel]) -> String {
    let mut prompt = format!(
        "Analysez les données suivantes d'un nœud Lightning Network et suggérez des actions d'optimisation :\n\n",
    );

    // Ajouter les statistiques du nœud
    prompt.push_str(&format!(
        "Statistiques du nœud :\n\
        - Alias : {}\n\
        - Nombre de canaux : {}\n\
        - Capacité totale : {} sats\n\
        - Balance locale totale : {} sats\n\
        - Balance distante totale : {} sats\n\
        - Taille moyenne des canaux : {} sats\n\
        - Uptime : {}%\n\n",
        stats.alias,
        stats.num_channels,
        stats.total_capacity,
        stats.total_local_balance,
        stats.total_remote_balance,
        stats.avg_channel_size,
        stats.uptime_percentage
    ));

    // Ajouter les informations sur les canaux
    prompt.push_str("Canaux :\n");
    for channel in channels {
        prompt.push_str(&format!(
            "- Canal {} avec {} :\n\
            \t- Capacité : {} sats\n\
            \t- Balance locale : {} sats\n\
            \t- Balance distante : {} sats\n\
            \t- Frais : {} msats + {} ppm\n\
            \t- Actif : {}\n\
            \t- Uptime : {}%\n",
            channel.channel_id,
            channel.remote_alias,
            channel.capacity,
            channel.local_balance,
            channel.remote_balance,
            channel.commit_fee,
            channel.fee_per_kw,
            channel.active,
            channel.uptime_percentage
        ));
    }

    prompt.push_str("\nSuggérez des actions concrètes pour optimiser ce nœud, en incluant pour chaque action :\n\
        1. Le type d'action (ajustement de frais, création de canal, fermeture de canal, rééquilibrage)\n\
        2. Une description détaillée\n\
        3. La priorité (haute, moyenne, basse)\n\
        4. L'impact attendu\n");

    prompt
}

fn parse_ai_response(response: &str) -> Result<Vec<Action>, String> {
    debug!("Parsing AI response");
    let mut actions = Vec::new();
    let mut current_action: Option<String> = None;
    let mut action_type = None;
    let mut description = String::new();
    let mut priority = None;
    let mut impact = String::new();

    for line in response.lines() {
        let line = line.trim();
        if line.is_empty() {
            // Sauvegarder l'action précédente si elle existe
            if let (Some(t), Some(p)) = (action_type.take(), priority.take()) {
                actions.push(Action {
                    action_type: t,
                    description: description.trim().to_string(),
                    priority: p,
                    impact: impact.trim().to_string(),
                });
                description.clear();
                impact.clear();
            }
            continue;
        }

        if line.starts_with("Type:") || line.starts_with("Action:") {
            current_action = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
            action_type = current_action.clone();
        } else if line.starts_with("Priority:") || line.starts_with("Priorité:") {
            priority = Some(line.split(':').nth(1).unwrap_or("").trim().to_string());
        } else if line.starts_with("Impact:") {
            impact = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else {
            description.push_str(line);
            description.push('\n');
        }
    }

    // Ajouter la dernière action si elle existe
    if let (Some(t), Some(p)) = (action_type, priority) {
        actions.push(Action {
            action_type: t,
            description: description.trim().to_string(),
            priority: p,
            impact: impact.trim().to_string(),
        });
    }

    if actions.is_empty() {
        return Err("Aucune action n'a pu être extraite de la réponse".to_string());
    }

    Ok(actions)
} 