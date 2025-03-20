use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use crate::types::{NodeStats, ChannelRecommendation, OutboundLiquidityValue, SuggestedFees, Channel, Recommendation};

const NODE_PUBKEY: &str = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
const BASE_URL: &str = "https://api.sparkseer.space";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub async fn fetch_node_stats() -> Result<NodeStats, String> {
    let url = format!("{}/node/{}", BASE_URL, NODE_PUBKEY);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_channel_recommendations() -> Result<Vec<ChannelRecommendation>, String> {
    let url = format!("{}/node/{}/channel_recommendations", BASE_URL, NODE_PUBKEY);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_outbound_liquidity_value() -> Result<OutboundLiquidityValue, String> {
    let url = format!("{}/node/{}/outbound_liquidity_value", BASE_URL, NODE_PUBKEY);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_suggested_fees() -> Result<SuggestedFees, String> {
    let url = format!("{}/node/{}/suggested_fees", BASE_URL, NODE_PUBKEY);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_channels() -> Result<Vec<Channel>, String> {
    let url = format!("{}/node/{}/channels", BASE_URL, NODE_PUBKEY);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_all_data() -> Result<(NodeStats, Vec<ChannelRecommendation>, OutboundLiquidityValue, SuggestedFees), String> {
    let stats = fetch_node_stats().await?;
    let recommendations = fetch_channel_recommendations().await?;
    let liquidity = fetch_outbound_liquidity_value().await?;
    let fees = fetch_suggested_fees().await?;

    Ok((stats, recommendations, liquidity, fees))
}

pub async fn get_ai_recommendations(stats: &NodeStats, channels: &[Channel]) -> Result<Vec<Recommendation>, String> {
    let prompt = format!(
        "Analysez les statistiques suivantes du nœud Lightning et fournissez des recommandations pour optimiser sa rentabilité:\n\
        Capacité totale: {} sats\n\
        Nombre de canaux: {}\n\
        Score de flexibilité: {}\n\
        Rangs: betweenness={}, closeness={}, eigenvector={}\n\
        Frais moyens: base={} msats, rate={} ppm\n\
        Nombre de canaux: {}\n\
        Fournissez 3 recommandations prioritaires avec leur impact estimé.",
        stats.total_capacity,
        stats.num_channels,
        stats.liquidity_flexibility_score,
        stats.betweenness_rank,
        stats.closeness_rank,
        stats.eigenvector_rank,
        stats.mean_outbound_base_fee,
        stats.mean_outbound_fee_rate,
        channels.len()
    );

    // Note: L'implémentation complète nécessiterait une clé API OpenAI
    // Pour le moment, nous retournons des recommandations statiques
    Ok(vec![
        Recommendation {
            id: "1".to_string(),
            title: "Optimisation des frais".to_string(),
            description: "Ajuster les frais de base et les taux pour maximiser la rentabilité tout en restant compétitif".to_string(),
            priority: crate::types::Priority::High,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        Recommendation {
            id: "2".to_string(),
            title: "Équilibrage de la liquidité".to_string(),
            description: "Rééquilibrer les canaux pour améliorer le score de flexibilité".to_string(),
            priority: crate::types::Priority::Medium,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
        Recommendation {
            id: "3".to_string(),
            title: "Expansion du réseau".to_string(),
            description: "Créer de nouveaux canaux avec des nœuds stratégiques pour améliorer le rang de betweenness".to_string(),
            priority: crate::types::Priority::Low,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        },
    ])
} 