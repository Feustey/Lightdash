use super::{NodeMetrics, ChannelMetrics};
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use tokio::time::{interval, Duration};

pub struct DataCollector {
    db: Pool<Postgres>,
    sparkseer_client: reqwest::Client,
    node_pubkey: String,
}

impl DataCollector {
    pub fn new(db: Pool<Postgres>, node_pubkey: String) -> Self {
        Self {
            db,
            sparkseer_client: reqwest::Client::new(),
            node_pubkey,
        }
    }

    pub async fn start_collection(&self) {
        let mut interval = interval(Duration::from_secs(3600)); // Collecter toutes les heures

        loop {
            interval.tick().await;
            if let Err(e) = self.collect_metrics().await {
                eprintln!("Erreur lors de la collecte des métriques : {}", e);
            }
        }
    }

    async fn collect_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        let now = Utc::now();
        
        // Collecter les métriques du nœud
        let node_metrics = self.fetch_node_metrics().await?;
        self.save_node_metrics(&node_metrics).await?;

        // Collecter les métriques des canaux
        let channel_metrics = self.fetch_channel_metrics().await?;
        for metrics in channel_metrics {
            self.save_channel_metrics(&metrics).await?;
        }

        Ok(())
    }

    async fn fetch_node_metrics(&self) -> Result<NodeMetrics, Box<dyn std::error::Error>> {
        let url = format!("https://api.sparkseer.space/v1/node/{}/stats", self.node_pubkey);
        let response = self.sparkseer_client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    async fn fetch_channel_metrics(&self) -> Result<Vec<ChannelMetrics>, Box<dyn std::error::Error>> {
        let url = format!("https://api.sparkseer.space/v1/node/{}/channels", self.node_pubkey);
        let response = self.sparkseer_client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    async fn save_node_metrics(&self, metrics: &NodeMetrics) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO node_metrics (
                timestamp, total_capacity, num_channels, total_volume,
                total_fees, avg_channel_size, network_centrality
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            metrics.timestamp,
            metrics.total_capacity as i64,
            metrics.num_channels as i32,
            metrics.total_volume as i64,
            metrics.total_fees as i64,
            metrics.avg_channel_size as i64,
            metrics.network_centrality
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn save_channel_metrics(&self, metrics: &ChannelMetrics) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO channel_metrics (
                channel_id, timestamp, local_balance, remote_balance,
                num_forwards, volume_forwarded, fees_earned, uptime
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            metrics.channel_id,
            metrics.timestamp,
            metrics.local_balance as i64,
            metrics.remote_balance as i64,
            metrics.num_forwards as i32,
            metrics.volume_forwarded as i64,
            metrics.fees_earned as i64,
            metrics.uptime
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn get_training_data(&self, start_date: DateTime<Utc>) -> Result<(Vec<NodeMetrics>, Vec<ChannelMetrics>), sqlx::Error> {
        let node_metrics = sqlx::query_as!(
            NodeMetrics,
            r#"
            SELECT * FROM node_metrics
            WHERE timestamp >= $1
            ORDER BY timestamp ASC
            "#,
            start_date
        )
        .fetch_all(&self.db)
        .await?;

        let channel_metrics = sqlx::query_as!(
            ChannelMetrics,
            r#"
            SELECT * FROM channel_metrics
            WHERE timestamp >= $1
            ORDER BY timestamp ASC
            "#,
            start_date
        )
        .fetch_all(&self.db)
        .await?;

        Ok((node_metrics, channel_metrics))
    }
} 