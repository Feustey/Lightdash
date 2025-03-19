use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub alias: String,
    pub num_channels: u32,
    pub total_capacity: u64,
    pub avg_channel_size: u64,
    pub total_fees_earned: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
}

impl std::fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelStatus::Active => write!(f, "Active"),
            ChannelStatus::Inactive => write!(f, "Inactive"),
            ChannelStatus::Pending => write!(f, "Pending"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: String,
    pub capacity: u64,
    pub status: ChannelStatus,
    pub remote_alias: String,
    pub uptime: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub description: String,
    pub severity: ImpactSeverity,
    pub action_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub channels: u32,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeeSimulation {
    pub base_fee: u64,
    pub fee_rate: u64,
    pub estimated_revenue: u64,
    pub estimated_routing_volume: u64,
    pub estimated_success_rate: f64,
    pub competitive_score: f64,
    pub potential_new_peers: u32,
    pub revenue_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationResult {
    pub estimated_revenue: u64,
    pub routing_volume: u64,
    pub success_rate: f64,
    pub competitiveness_score: f64,
    pub impacts: Vec<SimulationImpact>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationImpact {
    pub metric: String,
    pub change_percent: f64,
    pub description: String,
} 