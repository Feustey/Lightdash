#[cfg(test)]
mod tests {
    use crate::services::{parse_recommendations, NodeStats, Channel, Recommendation};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_parse_recommendations() {
        let test_content = r#"
        Recommandation 1: Ajuster les frais
        - Action: Augmenter les frais de base à 1000 sats
        - Impact: Élevé
        - Justification: Les frais actuels sont trop bas

        Recommandation 2: Rééquilibrer les canaux
        - Action: Déplacer 1M sats vers le canal X
        - Impact: Moyen
        - Justification: Déséquilibre de liquidité
        "#;

        let recommendations = parse_recommendations(test_content);
        assert!(!recommendations.is_empty());
        assert_eq!(recommendations.len(), 2);
        assert_eq!(recommendations[0].title, "Ajuster les frais");
        assert!(recommendations[0].priority > 0);
    }

    #[wasm_bindgen_test]
    async fn test_node_stats_validation() {
        let stats = NodeStats {
            total_capacity: 1_000_000,
            num_channels: 10,
            mean_channel_capacity: 100_000,
            median_channel_capacity: 90_000,
            mean_outbound_base_fee: 1000,
            mean_outbound_fee_rate: 100,
            median_outbound_base_fee: 900,
            median_outbound_fee_rate: 90,
            liquidity_flexibility_score: 0.8,
            betweenness_rank: 1000,
            closeness_rank: 900,
            eigenvector_rank: 800,
            node_alias: "test_node".to_string(),
            node_country: "FR".to_string(),
            inbound_liquidity_ratio: 0.5,
            outbound_liquidity_ratio: 0.5,
        };

        assert!(stats.total_capacity > 0);
        assert!(stats.num_channels > 0);
        assert!(stats.mean_channel_capacity <= stats.total_capacity);
        assert!(stats.inbound_liquidity_ratio + stats.outbound_liquidity_ratio <= 1.0);
    }

    #[wasm_bindgen_test]
    async fn test_channel_validation() {
        let channel = Channel {
            remote_pubkey: "test_pubkey".to_string(),
            capacity: 1_000_000,
            local_balance: 500_000,
            remote_balance: 500_000,
            is_active: true,
        };

        assert_eq!(channel.local_balance + channel.remote_balance, channel.capacity);
        assert!(!channel.remote_pubkey.is_empty());
    }
} 