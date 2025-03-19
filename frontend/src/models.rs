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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recommendation {
    pub title: String,
    pub description: String,
    pub severity: RecommendationSeverity,
    pub action_type: ActionType,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationResult {
    pub estimated_revenue: u64,
    pub current_revenue: u64,
    pub routing_impact: Impact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SparkSeerStats {
    pub total_capacity: u64,
    pub active_channels: u64,
    pub avg_fee_rate: f64,
    pub success_rate: f64,
    pub routing_volume: u64,
    pub num_channels: u64,
    pub health_score: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeHistory {
    pub timestamp: u64,
    pub base_fee: u64,
    pub fee_rate: f64,
    pub revenue: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeerComparison {
    pub peer_pubkey: String,
    pub fee_rate: f64,
    pub base_fee: u64,
    pub success_rate: f64,
    pub volume: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPeer {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub channels: u64,
    pub score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecommendationSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    UpdateFees,
    OpenChannel,
    CloseChannel,
    Rebalance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Impact {
    pub description: String,
    pub severity: ImpactSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ImpactSeverity {
    Positive,
    Neutral,
    Negative,
} 