use wasm_bindgen::JsValue;
use web_sys::window;
use crate::models::*;

#[derive(Clone)]
pub struct ApiService {
    base_url: String,
}

impl PartialEq for ApiService {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
    }
}

impl ApiService {
    pub fn new() -> Self {
        let base_url = window()
            .expect("La fenêtre devrait être disponible")
            .location()
            .origin()
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        Self { base_url }
    }

    pub async fn get_stats(&self) -> Result<SparkSeerStats, JsValue> {
        // Simulation de données pour le développement
        Ok(SparkSeerStats {
            active_channels: 42,
            total_capacity: 1_000_000,
            monthly_revenue: 50_000,
        })
    }

    pub async fn get_fee_history(&self) -> Result<FeeHistory, JsValue> {
        // Simulation de données pour le développement
        Ok(FeeHistory {
            entries: vec![
                FeeHistoryEntry {
                    date: "2024-03-19".to_string(),
                    revenue: 5000,
                },
                FeeHistoryEntry {
                    date: "2024-03-18".to_string(),
                    revenue: 4800,
                },
                FeeHistoryEntry {
                    date: "2024-03-17".to_string(),
                    revenue: 5200,
                },
            ],
        })
    }

    pub async fn get_peer_comparison(&self) -> Result<PeerComparison, JsValue> {
        // Simulation de données pour le développement
        Ok(PeerComparison {
            suggested_peers: vec![
                SuggestedPeer {
                    alias: "Node1".to_string(),
                    similarity: 0.85,
                },
                SuggestedPeer {
                    alias: "Node2".to_string(),
                    similarity: 0.75,
                },
                SuggestedPeer {
                    alias: "Node3".to_string(),
                    similarity: 0.65,
                },
            ],
        })
    }

    pub async fn get_ai_recommendations(&self) -> Result<Vec<Recommendation>, JsValue> {
        // Simulation de données pour le développement
        Ok(vec![
            Recommendation {
                title: "Optimiser les frais du canal 123".to_string(),
                description: "Les frais actuels sont trop élevés, ce qui réduit le routage".to_string(),
                severity: RecommendationSeverity::High,
                channel_id: Some("123".to_string()),
            },
            Recommendation {
                title: "Augmenter la capacité du canal 456".to_string(),
                description: "Le canal est souvent saturé".to_string(),
                severity: RecommendationSeverity::Medium,
                channel_id: Some("456".to_string()),
            },
        ])
    }

    pub async fn simulate_fees(&self, channel_id: String, fee_rate: f64) -> Result<SimulationResult, JsValue> {
        // Simulation de données pour le développement
        Ok(SimulationResult {
            current_revenue: 5000,
            routing_impact: -0.15, // -15% impact sur le routage
        })
    }
} 