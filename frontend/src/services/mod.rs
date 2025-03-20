use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::types::{NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees, Channel, Recommendation};
use wasm_bindgen::JsValue;
use web_sys::console;
use std::sync::Once;
use std::sync::Mutex;
use std::collections::HashMap;

const NODE_PUBKEY: &str = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
const BASE_URL: &str = "https://api.sparkseer.space";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u32 = 1000;

static API_KEYS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
static INIT: Once = Once::new();

fn init_api_keys() {
    INIT.call_once(|| {
        if let Ok(mut keys) = API_KEYS.lock() {
            if let Ok(sparkseer_key) = std::env::var("SPARKSEER_API_KEY") {
                keys.insert("sparkseer".to_string(), sparkseer_key);
            }
            if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
                keys.insert("openai".to_string(), openai_key);
            }
            if let Ok(deepseek_key) = std::env::var("DEEPSEEK_API_KEY") {
                keys.insert("deepseek".to_string(), deepseek_key);
            }
        }
    });
}

fn get_api_key(service: &str) -> Option<String> {
    init_api_keys();
    API_KEYS.lock().ok()?.get(service).cloned()
}

fn log_api_call(endpoint: &str, status: u16, error: Option<&str>) {
    let message = if let Some(err) = error {
        format!("API Call to {} failed with status {}: {}", endpoint, status, err)
    } else {
        format!("API Call to {} successful with status {}", endpoint, status)
    };
    console::log_1(&JsValue::from_str(&message));
}

async fn retry_request<T, F>(f: F) -> Result<T, String>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, String>> + '_>>,
{
    let mut retries = 0;
    let mut last_error = None;

    while retries < MAX_RETRIES {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                retries += 1;
                if retries < MAX_RETRIES {
                    gloo_timers::future::TimeoutFuture::new(RETRY_DELAY_MS * retries).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error".to_string()))
}

fn create_request(url: &str) -> Request {
    let mut request = Request::get(url);
    if let Some(api_key) = get_api_key("sparkseer") {
        request = request.header("Authorization", &format!("Bearer {}", api_key));
    }
    request
}

pub async fn fetch_node_stats() -> Result<NodeStats, String> {
    let url = format!("{}/node/{}", BASE_URL, NODE_PUBKEY);
    console::log_1(&JsValue::from_str(&format!("Fetching node stats from {}", url)));
    
    retry_request(|| Box::pin(async {
        match create_request(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.json().await {
                    Ok(stats) => {
                        log_api_call("node_stats", status, None);
                        Ok(stats)
                    }
                    Err(e) => {
                        log_api_call("node_stats", status, Some(&e.to_string()));
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                log_api_call("node_stats", 0, Some(&e.to_string()));
                Err(e.to_string())
            }
        }
    })).await
}

pub async fn fetch_channel_recommendations() -> Result<Vec<ChannelRecommendation>, String> {
    let url = format!("{}/node/{}/channel_recommendations", BASE_URL, NODE_PUBKEY);
    console::log_1(&JsValue::from_str(&format!("Fetching channel recommendations from {}", url)));
    
    retry_request(|| Box::pin(async {
        match create_request(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.json().await {
                    Ok(recommendations) => {
                        log_api_call("channel_recommendations", status, None);
                        Ok(recommendations)
                    }
                    Err(e) => {
                        log_api_call("channel_recommendations", status, Some(&e.to_string()));
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                log_api_call("channel_recommendations", 0, Some(&e.to_string()));
                Err(e.to_string())
            }
        }
    })).await
}

pub async fn fetch_outbound_liquidity_value() -> Result<OutboundLiquidityValue, String> {
    let url = format!("{}/node/{}/outbound_liquidity_value", BASE_URL, NODE_PUBKEY);
    console::log_1(&JsValue::from_str(&format!("Fetching outbound liquidity value from {}", url)));
    
    retry_request(|| Box::pin(async {
        match create_request(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.json().await {
                    Ok(liquidity) => {
                        log_api_call("outbound_liquidity", status, None);
                        Ok(liquidity)
                    }
                    Err(e) => {
                        log_api_call("outbound_liquidity", status, Some(&e.to_string()));
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                log_api_call("outbound_liquidity", 0, Some(&e.to_string()));
                Err(e.to_string())
            }
        }
    })).await
}

pub async fn fetch_suggested_fees() -> Result<SuggestedFees, String> {
    let url = format!("{}/node/{}/suggested_fees", BASE_URL, NODE_PUBKEY);
    console::log_1(&JsValue::from_str(&format!("Fetching suggested fees from {}", url)));
    
    retry_request(|| Box::pin(async {
        match create_request(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.json().await {
                    Ok(fees) => {
                        log_api_call("suggested_fees", status, None);
                        Ok(fees)
                    }
                    Err(e) => {
                        log_api_call("suggested_fees", status, Some(&e.to_string()));
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                log_api_call("suggested_fees", 0, Some(&e.to_string()));
                Err(e.to_string())
            }
        }
    })).await
}

pub async fn fetch_channels() -> Result<Vec<Channel>, String> {
    let url = format!("{}/node/{}/channels", BASE_URL, NODE_PUBKEY);
    console::log_1(&JsValue::from_str(&format!("Fetching channels from {}", url)));
    
    retry_request(|| Box::pin(async {
        match create_request(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match response.json().await {
                    Ok(channels) => {
                        log_api_call("channels", status, None);
                        Ok(channels)
                    }
                    Err(e) => {
                        log_api_call("channels", status, Some(&e.to_string()));
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                log_api_call("channels", 0, Some(&e.to_string()));
                Err(e.to_string())
            }
        }
    })).await
}

pub async fn fetch_all_data() -> Result<(NodeStats, Vec<ChannelRecommendation>, OutboundLiquidityValue, SuggestedFees), String> {
    let stats = fetch_node_stats().await?;
    let recommendations = fetch_channel_recommendations().await?;
    let liquidity = fetch_outbound_liquidity_value().await?;
    let fees = fetch_suggested_fees().await?;

    Ok((stats, recommendations, liquidity, fees))
}

pub async fn get_ai_recommendations(stats: &NodeStats, channels: &[Channel]) -> Result<Vec<Recommendation>, String> {
    // Validation des données d'entrée
    if channels.is_empty() {
        return Err("Aucun canal disponible pour l'analyse".to_string());
    }

    // Préparation des données des canaux avec validation
    let mut channel_details = String::new();
    let mut total_inbound = 0;
    let mut total_outbound = 0;
    let mut active_channels = 0;
    let mut inactive_channels = 0;

    for channel in channels {
        // Validation des données du canal
        if channel.capacity == 0 {
            console::warn_1(&JsValue::from_str(&format!(
                "Canal {} avec capacité nulle détecté",
                channel.remote_pubkey
            )));
            continue;
        }

        channel_details.push_str(&format!(
            "  - {} : {} sats (Local: {} sats, Remote: {} sats)\n",
            channel.remote_pubkey,
            channel.capacity,
            channel.local_balance,
            channel.remote_balance
        ));

        total_inbound += channel.remote_balance;
        total_outbound += channel.local_balance;
        if channel.is_active {
            active_channels += 1;
        } else {
            inactive_channels += 1;
        }
    }

    // Calcul des métriques avec validation
    let inbound_ratio = if stats.total_capacity > 0 {
        (total_inbound as f64 / stats.total_capacity as f64) * 100.0
    } else {
        0.0
    };
    
    let outbound_ratio = if stats.total_capacity > 0 {
        (total_outbound as f64 / stats.total_capacity as f64) * 100.0
    } else {
        0.0
    };
    
    let active_ratio = if !channels.is_empty() {
        (active_channels as f64 / channels.len() as f64) * 100.0
    } else {
        0.0
    };

    let prompt = format!(
        "Tu es un expert en analyse des performances des nœuds Lightning Network et en optimisation de la rentabilité des canaux. 

### 🔍 Contexte :
Je possède un nœud Lightning et je souhaite optimiser ses performances et sa rentabilité. Voici les données récupérées depuis **1ML** et **Sparkseer** concernant mon nœud :

#### 📡 Données issues de 1ML :
- **Capacité totale** : {} sats
- **Nombre de canaux ouverts** : {} ({} actifs, {} inactifs)
- **Liste des canaux avec leurs capacités** :
{}
- **Score de connectivité** : {:.2}
- **Centralité du nœud dans le graphe du réseau** : {}
- **Alias du nœud** : {}
- **Pays d'hébergement** : {}
- **Politique de frais (fee policy) moyenne** :
  - Base fee : {} msats
  - Fee rate : {} ppm

#### 📊 Données issues de Sparkseer :
- **Utilisation des canaux** :
  - Score de flexibilité : {:.2}
  - Rangs : betweenness={}, closeness={}, eigenvector={}
  - Taux de canaux actifs : {:.1}%
- **Flux de liquidité** :
  - Capacité moyenne : {:.2} sats
  - Capacité médiane : {} sats
  - Ratio liquidité entrante : {:.1}%
  - Ratio liquidité sortante : {:.1}%
- **Frais actuels** :
  - Base fee médian : {} msats
  - Fee rate médian : {} ppm
  - Base fee moyen : {} msats
  - Fee rate moyen : {} ppm

### 🎯 Objectif :
- Maximiser la rentabilité en ajustant les fees de manière optimale
- Éviter l'épuisement des liquidités tout en gardant des canaux actifs
- Fermer ou rééquilibrer les canaux peu performants
- Identifier les meilleurs pairs pour ouvrir de nouveaux canaux

### 🚀 Ta mission :
À partir des données ci-dessus, analyse la situation et propose une liste de **recommandations détaillées** classées par priorité.  
Chaque recommandation doit être claire, actionable et justifiée par les données fournies.  
Pour chaque recommandation, indique :
1. L'action précise à effectuer
2. La justification basée sur les données
3. L'impact estimé (faible, moyen, élevé)
4. Les métriques à surveiller après l'action",
        stats.total_capacity,
        stats.num_channels,
        active_channels,
        inactive_channels,
        channel_details,
        stats.liquidity_flexibility_score,
        stats.betweenness_rank,
        stats.node_alias,
        stats.node_country,
        stats.mean_outbound_base_fee,
        stats.mean_outbound_fee_rate,
        stats.liquidity_flexibility_score,
        stats.betweenness_rank,
        stats.closeness_rank,
        stats.eigenvector_rank,
        active_ratio,
        stats.mean_channel_capacity,
        stats.median_channel_capacity,
        inbound_ratio,
        outbound_ratio,
        stats.median_outbound_base_fee,
        stats.median_outbound_fee_rate,
        stats.mean_outbound_base_fee,
        stats.mean_outbound_fee_rate
    );

    // Requête OpenAI
    let openai_request = Request::post(OPENAI_API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", get_api_key("openai").unwrap_or_default()))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [
                {
                    "role": "system",
                    "content": "Tu es un expert en analyse des performances des nœuds Lightning Network et en optimisation de la rentabilité des canaux. Tu fournis des recommandations détaillées, actionables et basées sur les données."
                },
                {
                    "role": "user",
                    "content": prompt.clone()
                }
            ],
            "temperature": 0.7,
            "max_tokens": 1500
        }))
        .map_err(|e| e.to_string())?;

    // Requête DeepSeek
    let deepseek_request = Request::post(DEEPSEEK_API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", get_api_key("deepseek").unwrap_or_default()))
        .json(&serde_json::json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": "Tu es un expert en analyse des performances des nœuds Lightning Network et en optimisation de la rentabilité des canaux. Tu fournis des recommandations détaillées, actionables et basées sur les données."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 1500
        }))
        .map_err(|e| e.to_string())?;

    // Exécution parallèle des requêtes
    let (openai_result, deepseek_result) = futures::future::join(
        retry_request(|| Box::pin(async {
            match openai_request.send().await {
                Ok(response) => {
                    if response.ok() {
                        match response.json::<serde_json::Value>().await {
                            Ok(json) => {
                                if let Some(choices) = json.get("choices") {
                                    if let Some(first) = choices.get(0) {
                                        if let Some(message) = first.get("message") {
                                            if let Some(content) = message.get("content") {
                                                let recommendations = parse_recommendations(content.as_str().unwrap_or_default());
                                                return Ok(recommendations);
                                            }
                                        }
                                    }
                                }
                                Err("Format de réponse OpenAI invalide".to_string())
                            }
                            Err(e) => Err(format!("Erreur de parsing JSON OpenAI: {}", e))
                        }
                    } else {
                        Err(format!("Erreur API OpenAI: {}", response.status()))
                    }
                }
                Err(e) => Err(format!("Erreur de requête OpenAI: {}", e))
            }
        })),
        retry_request(|| Box::pin(async {
            match deepseek_request.send().await {
                Ok(response) => {
                    if response.ok() {
                        match response.json::<serde_json::Value>().await {
                            Ok(json) => {
                                if let Some(choices) = json.get("choices") {
                                    if let Some(first) = choices.get(0) {
                                        if let Some(message) = first.get("message") {
                                            if let Some(content) = message.get("content") {
                                                let recommendations = parse_recommendations(content.as_str().unwrap_or_default());
                                                return Ok(recommendations);
                                            }
                                        }
                                    }
                                }
                                Err("Format de réponse DeepSeek invalide".to_string())
                            }
                            Err(e) => Err(format!("Erreur de parsing JSON DeepSeek: {}", e))
                        }
                    } else {
                        Err(format!("Erreur API DeepSeek: {}", response.status()))
                    }
                }
                Err(e) => Err(format!("Erreur de requête DeepSeek: {}", e))
            }
        }))
    ).await;

    // Fusion et déduplication des recommandations
    let mut all_recommendations = Vec::new();
    let mut seen_recommendations = std::collections::HashSet::new();

    // Ajouter les recommandations d'OpenAI
    if let Ok(openai_recs) = openai_result {
        for rec in openai_recs {
            let key = format!("{}_{}", rec.title, rec.description);
            if !seen_recommendations.contains(&key) {
                seen_recommendations.insert(key);
                all_recommendations.push(rec);
            }
        }
    }

    // Ajouter les recommandations de DeepSeek
    if let Ok(deepseek_recs) = deepseek_result {
        for rec in deepseek_recs {
            let key = format!("{}_{}", rec.title, rec.description);
            if !seen_recommendations.contains(&key) {
                seen_recommendations.insert(key);
                all_recommendations.push(rec);
            }
        }
    }

    // Trier par priorité
    all_recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));

    Ok(all_recommendations)
}

fn parse_recommendations(content: &str) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();
    let mut id = 1;
    let mut current_recommendation = String::new();
    let mut current_impact = crate::types::Priority::Low;

    for line in content.lines() {
        if line.starts_with("-") {
            // Si nous avons une recommandation en cours, l'ajouter
            if !current_recommendation.is_empty() {
                recommendations.push(Recommendation {
                    id: id.to_string(),
                    title: current_recommendation.split(":").next().unwrap_or("Recommandation").to_string(),
                    description: current_recommendation.clone(),
                    priority: current_impact,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    updated_at: chrono::Utc::now().to_rfc3339(),
                });
                id += 1;
                current_recommendation.clear();
            }

            // Démarrer une nouvelle recommandation
            current_recommendation = line.trim_start_matches("- ").to_string();
            
            // Détecter l'impact
            current_impact = if current_recommendation.contains("impact élevé") || 
                               current_recommendation.contains("priorité haute") ||
                               current_recommendation.contains("important") {
                crate::types::Priority::High
            } else if current_recommendation.contains("impact moyen") || 
                      current_recommendation.contains("priorité moyenne") {
                crate::types::Priority::Medium
            } else {
                crate::types::Priority::Low
            };
        } else if !line.trim().is_empty() && !current_recommendation.is_empty() {
            // Ajouter les détails à la recommandation en cours
            current_recommendation.push_str("\n");
            current_recommendation.push_str(line.trim());
        }
    }

    // Ajouter la dernière recommandation si elle existe
    if !current_recommendation.is_empty() {
        recommendations.push(Recommendation {
            id: id.to_string(),
            title: current_recommendation.split(":").next().unwrap_or("Recommandation").to_string(),
            description: current_recommendation,
            priority: current_impact,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });
    }

    recommendations
} 