use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub remote_pubkey: String,
    pub status: ChannelStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: i64,
    pub type_: TransactionType,
    pub status: TransactionStatus,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Payment,
    Invoice,
    ChannelOpen,
    ChannelClose,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
    Completed,
    Pending,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub channels: u32,
    pub version: String,
} 