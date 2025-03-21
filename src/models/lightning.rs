use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: String,
    pub remote_pubkey: String,
    pub remote_alias: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub commit_fee: u64,
    pub fee_per_kw: u64,
    pub unsettled_balance: u64,
    pub total_satoshis_sent: u64,
    pub total_satoshis_received: u64,
    pub num_updates: u64,
    pub csv_delay: u32,
    pub private: bool,
    pub active: bool,
    pub uptime_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: i64,
    pub type_: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Payment,
    Invoice,
    ChannelOpen,
    ChannelClose,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Completed,
    Pending,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub channels: u32,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutboundLiquidityValue {
    pub total_value: u64,
    pub utilization_rate: f64,
    pub value_per_channel: Vec<Channel>,
} 