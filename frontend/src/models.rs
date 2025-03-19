use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub channels: u32,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub remote_pubkey: String,
    pub status: ChannelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
    Closing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: DateTime<Utc>,
    pub type_: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Payment,
    Invoice,
    ChannelOpen,
    ChannelClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub current: FeeSimulation,
    pub simulated: FeeSimulation,
    pub recommendation: String,
    pub impact_analysis: Vec<SimulationImpact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationImpact {
    pub metric: String,
    pub change: f64,
    pub description: String,
    pub severity: ImpactSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactSeverity {
    Positive,
    Neutral,
    Negative,
}

} 