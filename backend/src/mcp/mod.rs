use tch::{nn, Device, Tensor};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::services::amboss::AmbossService;
use crate::services::sparkseer::SparkseerService;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelMetrics {
    pub channel_id: String,
    pub capacity: f64,
    pub local_balance: f64,
    pub remote_balance: f64,
    pub last_update: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub pubkey: String,
    pub alias: Option<String>,
    pub total_capacity: f64,
    pub channel_count: i32,
    pub first_seen: String,
    pub last_update: String,
    pub channels: Vec<ChannelMetrics>,
}

pub struct LightningMCP {
    model: nn::Sequential,
    device: Device,
    sparkseer: SparkseerService,
    amboss: AmbossService,
}

impl LightningMCP {
    pub fn new(sparkseer_api_key: String, amboss_api_key: String) -> Result<Self> {
        let device = Device::cuda_if_available();
        let vs = nn::VarStore::new(device);
        let p = vs.root();

        // Définition du modèle
        let model = nn::seq()
            .add(nn::linear(&p, 14, 128, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&p, 128, 64, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&p, 64, 32, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(&p, 32, 4, Default::default())); // 4 sorties pour les différents types d'actions

        Ok(Self {
            model,
            device,
            sparkseer: SparkseerService::new(sparkseer_api_key),
            amboss: AmbossService::new(amboss_api_key),
        })
    }

    pub fn prepare_input(&self, node: &NodeMetrics, channels: &[ChannelMetrics]) -> Tensor {
        let mut features = Vec::new();

        // Caractéristiques du nœud
        features.push(node.total_capacity as f64);
        features.push(node.channel_count as f64);
        features.push(node.total_capacity as f64);
        features.push(node.total_capacity as f64);
        features.push(node.total_capacity as f64);
        features.push(node.total_capacity as f64);

        // Agrégation des métriques des canaux
        let mut total_local_balance = 0f64;
        let mut total_remote_balance = 0f64;
        let mut total_forwards = 0f64;
        let mut total_fees = 0f64;
        let mut avg_uptime = 0f64;

        for channel in channels {
            total_local_balance += channel.local_balance as f64;
            total_remote_balance += channel.remote_balance as f64;
            total_forwards += channel.num_forwards as f64;
            total_fees += channel.fees_earned as f64;
            avg_uptime += channel.uptime;
        }

        let num_channels = channels.len() as f64;
        if num_channels > 0.0 {
            avg_uptime /= num_channels;
        }

        features.push(total_local_balance);
        features.push(total_remote_balance);
        features.push(total_forwards);
        features.push(total_fees);
        features.push(avg_uptime);

        // Calcul de métriques dérivées
        let balance_ratio = if total_local_balance + total_remote_balance > 0.0 {
            total_local_balance / (total_local_balance + total_remote_balance)
        } else {
            0.5
        };
        features.push(balance_ratio);

        let fee_per_forward = if total_forwards > 0.0 {
            total_fees / total_forwards
        } else {
            0.0
        };
        features.push(fee_per_forward);

        // Conversion en tenseur
        Tensor::of_slice(&features).to_device(self.device)
    }

    pub fn predict(&self, input: &Tensor) -> Vec<f64> {
        let output = self.model.forward(input);
        let probabilities = output.softmax(-1, output.kind());
        probabilities.to_vec1().unwrap()
    }

    pub fn train(&mut self, inputs: &Tensor, targets: &Tensor, learning_rate: f64, epochs: i64) {
        let mut opt = nn::Adam::default().build(&self.model.parameters(), learning_rate).unwrap();

        for epoch in 0..epochs {
            let loss = self.model.forward(inputs).cross_entropy_for_logits(targets);
            opt.backward_step(&loss);

            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {}", epoch, f64::from(&loss));
            }
        }
    }

    pub async fn get_node_metrics(&self, pubkey: &str) -> Result<NodeMetrics> {
        // Récupération des données Sparkseer
        let sparkseer_data = self.sparkseer.get_node_stats(pubkey).await?;
        
        // Récupération des données Amboss
        let amboss_data = self.amboss.get_node_info(pubkey).await?;
        let amboss_channels = self.amboss.get_node_channels(pubkey).await?;

        // Fusion des données
        let channels = amboss_channels.into_iter().map(|channel| ChannelMetrics {
            channel_id: channel.id,
            capacity: channel.capacity as f64,
            local_balance: 0.0, // À compléter avec les données Sparkseer si disponibles
            remote_balance: 0.0, // À compléter avec les données Sparkseer si disponibles
            last_update: channel.last_update,
            status: channel.status,
        }).collect();

        Ok(NodeMetrics {
            pubkey: pubkey.to_string(),
            alias: amboss_data.alias,
            total_capacity: amboss_data.capacity as f64,
            channel_count: amboss_data.channel_count,
            first_seen: amboss_data.first_seen,
            last_update: amboss_data.updated_at,
            channels,
        })
    }

    pub async fn generate_recommendations(&self, pubkey: &str) -> Result<Vec<Recommendation>> {
        let metrics = self.get_node_metrics(pubkey).await?;
        let input = self.prepare_input(&metrics, &metrics.channels);
        let predictions = self.predict(&input)?;
        
        let mut recommendations = Vec::new();
        
        // Analyse des prédictions pour générer des recommandations
        if predictions[0] > 0.7 {
            recommendations.push(Recommendation {
                action: "Augmenter la capacité des canaux".to_string(),
                priority: Priority::High,
                reason: self.generate_reason(&predictions, 0),
            });
        }
        
        if predictions[1] > 0.7 {
            recommendations.push(Recommendation {
                action: "Optimiser la distribution des fonds".to_string(),
                priority: Priority::Medium,
                reason: self.generate_reason(&predictions, 1),
            });
        }
        
        if predictions[2] > 0.7 {
            recommendations.push(Recommendation {
                action: "Rééquilibrer les canaux".to_string(),
                priority: Priority::High,
                reason: self.generate_reason(&predictions, 2),
            });
        }
        
        Ok(recommendations)
    }

    fn generate_reason(&self, predictions: &[f64], action_index: usize) -> String {
        match action_index {
            0 => format!(
                "Recommandation d'augmenter la capacité des canaux (confiance: {:.1}%).",
                predictions[0] * 100.0
            ),
            1 => format!(
                "Recommandation d'optimiser la distribution des fonds (confiance: {:.1}%).",
                predictions[1] * 100.0
            ),
            2 => format!(
                "Recommandation de rééquilibrer les canaux (confiance: {:.1}%).",
                predictions[2] * 100.0
            ),
            _ => String::from("Raison non spécifiée"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRecommendation {
    pub action_type: ActionType,
    pub priority: u32,
    pub confidence: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    OpenChannel,
    CloseChannel,
    UpdateFees,
    Rebalance,
}

impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::OpenChannel => "Ouvrir un canal",
            ActionType::CloseChannel => "Fermer un canal",
            ActionType::UpdateFees => "Mettre à jour les frais",
            ActionType::Rebalance => "Rééquilibrer",
        }.to_string()
    }
} 