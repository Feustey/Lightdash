use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NodeStats {
    pub alias: String,
    pub capacity: u64,
    pub channel_count: u32,
    pub active_channels: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Channel {
    pub id: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub status: ChannelStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub fee: u32,
    pub status: TransactionStatus,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: RecommendationSeverity,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RecommendationSeverity {
    High,
    Medium,
    Low,
} 