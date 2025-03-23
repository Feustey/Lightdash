use super::{LightningMCP, NodeMetrics, ChannelMetrics};
use tch::{Tensor, Device};
use chrono::{DateTime, Utc, Duration};
use sqlx::{Pool, Postgres};

pub struct ModelTrainer {
    mcp: LightningMCP,
    data_collector: super::data_collector::DataCollector,
}

impl ModelTrainer {
    pub fn new(db: Pool<Postgres>, node_pubkey: String) -> Self {
        Self {
            mcp: LightningMCP::new(),
            data_collector: super::data_collector::DataCollector::new(db, node_pubkey),
        }
    }

    pub async fn train_model(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Récupérer les données des 30 derniers jours
        let start_date = Utc::now() - Duration::days(30);
        let (node_metrics, channel_metrics) = self.data_collector.get_training_data(start_date).await?;

        // Préparer les données d'entraînement
        let (inputs, targets) = self.prepare_training_data(&node_metrics, &channel_metrics)?;

        // Entraîner le modèle
        self.mcp.train(&inputs, &targets, 0.001, 1000);

        Ok(())
    }

    fn prepare_training_data(&self, node_metrics: &[NodeMetrics], channel_metrics: &[ChannelMetrics]) 
        -> Result<(Tensor, Tensor), Box<dyn std::error::Error>> 
    {
        let mut input_data = Vec::new();
        let mut target_data = Vec::new();

        // Regrouper les métriques de canaux par timestamp
        let mut channel_metrics_by_time: std::collections::HashMap<DateTime<Utc>, Vec<&ChannelMetrics>> = 
            std::collections::HashMap::new();
        
        for metric in channel_metrics {
            channel_metrics_by_time
                .entry(metric.timestamp)
                .or_insert_with(Vec::new)
                .push(metric);
        }

        // Pour chaque point de données du nœud
        for (i, node_metric) in node_metrics.iter().enumerate() {
            if i + 1 >= node_metrics.len() {
                break; // Ignorer le dernier point car nous n'avons pas de cible
            }

            // Obtenir les métriques de canaux correspondantes
            let channels = channel_metrics_by_time
                .get(&node_metric.timestamp)
                .cloned()
                .unwrap_or_default();

            // Préparer l'entrée
            let input = self.mcp.prepare_input(node_metric, &channels);
            input_data.push(input);

            // Préparer la cible (basée sur les changements dans les métriques suivantes)
            let next_metric = &node_metrics[i + 1];
            let target = self.calculate_target(node_metric, next_metric);
            target_data.push(target);
        }

        // Convertir les vecteurs en tenseurs
        let device = Device::cuda_if_available();
        let inputs = Tensor::stack(&input_data, 0).to_device(device);
        let targets = Tensor::stack(&target_data, 0).to_device(device);

        Ok((inputs, targets))
    }

    fn calculate_target(&self, current: &NodeMetrics, next: &NodeMetrics) -> Tensor {
        let mut target = vec![0.0; 4];

        // Détecter les changements significatifs
        let capacity_change = (next.total_capacity as f64 - current.total_capacity as f64) / current.total_capacity as f64;
        let channel_change = next.num_channels as i32 - current.num_channels as i32;
        let fee_change = (next.total_fees as f64 - current.total_fees as f64) / current.total_fees as f64;
        let balance_change = (next.avg_channel_size as f64 - current.avg_channel_size as f64) / current.avg_channel_size as f64;

        // Règles pour déterminer les actions qui auraient dû être prises
        if channel_change > 0 {
            target[0] = 1.0; // OpenChannel
        }
        if channel_change < 0 {
            target[1] = 1.0; // CloseChannel
        }
        if fee_change.abs() > 0.1 {
            target[2] = 1.0; // UpdateFees
        }
        if balance_change.abs() > 0.2 {
            target[3] = 1.0; // Rebalance
        }

        Tensor::of_slice(&target)
    }

    pub fn save_model(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implémenter la sauvegarde du modèle
        Ok(())
    }

    pub fn load_model(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implémenter le chargement du modèle
        Ok(())
    }
} 