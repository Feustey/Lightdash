use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use prometheus::{IntCounterVec, IntGaugeVec, Opts, Registry};
use std::sync::Arc;

pub struct Metrics {
    pub registry: Registry,
    pub node_info_requests: IntCounterVec,
    pub channel_count: IntGaugeVec,
    pub total_capacity: IntGaugeVec,
    pub transaction_count: IntCounterVec,
    pub error_count: IntCounterVec,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        let registry = Registry::new();

        // Compteur pour les requêtes d'information du nœud
        let node_info_requests = IntCounterVec::new(
            Opts::new("node_info_requests_total", "Nombre total de requêtes d'information du nœud"),
            &["status"],
        ).unwrap();
        registry.register(Box::new(node_info_requests.clone())).unwrap();

        // Jauge pour le nombre de canaux
        let channel_count = IntGaugeVec::new(
            Opts::new("lightning_channels", "Nombre de canaux Lightning"),
            &["status"],
        ).unwrap();
        registry.register(Box::new(channel_count.clone())).unwrap();

        // Jauge pour la capacité totale
        let total_capacity = IntGaugeVec::new(
            Opts::new("lightning_total_capacity", "Capacité totale en satoshis"),
            &["type"],
        ).unwrap();
        registry.register(Box::new(total_capacity.clone())).unwrap();

        // Compteur pour les transactions
        let transaction_count = IntCounterVec::new(
            Opts::new("lightning_transactions_total", "Nombre total de transactions"),
            &["type", "status"],
        ).unwrap();
        registry.register(Box::new(transaction_count.clone())).unwrap();

        // Compteur pour les erreurs
        let error_count = IntCounterVec::new(
            Opts::new("lightning_errors_total", "Nombre total d'erreurs"),
            &["type"],
        ).unwrap();
        registry.register(Box::new(error_count.clone())).unwrap();

        Arc::new(Self {
            registry,
            node_info_requests,
            channel_count,
            total_capacity,
            transaction_count,
            error_count,
        })
    }

    pub fn setup_prometheus_middleware() -> PrometheusMetrics {
        PrometheusMetricsBuilder::new("lightdash")
            .endpoint("/metrics")
            .build()
            .unwrap()
    }
} 