use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub pubkey: String,
    pub total_capacity: u64,
    pub num_channels: u32,
    pub mean_channel_capacity: f64,
    pub median_channel_capacity: u64,
    pub mean_outbound_base_fee: u64,
    pub mean_outbound_fee_rate: u64,
    pub median_outbound_base_fee: u64,
    pub median_outbound_fee_rate: u64,
    pub mean_inbound_base_fee: u64,
    pub mean_inbound_fee_rate: u64,
    pub median_inbound_base_fee: u64,
    pub median_inbound_fee_rate: u64,
    pub alias: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub torv3: Option<String>,
    pub last_update: i64,
    pub betweenness_rank: u32,
    pub closeness_rank: u32,
    pub eigenvector_rank: u32,
    pub weighted_betweenness_rank: u32,
    pub weighted_closeness_rank: u32,
    pub weighted_eigenvector_rank: u32,
    pub htlc_response_time_mean: f64,
    pub htlc_response_time_median: f64,
    pub liquidity_flexibility_score: f64,
    pub effective_outbound_balance: f64,
    pub effective_inbound_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRecommendation {
    pub pubkey: String,
    pub info: Vec<ChannelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub gain_in_betweenness_rank: i32,
    pub gain_in_closeness_rank: i32,
    pub gain_in_eigenvector_rank: i32,
    pub minimum_viable_capacity: u64,
    pub ideal_capacity: u64,
    pub passive_fee_ppm: u32,
    pub active_fee_ppm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundLiquidityValue {
    pub channel_peers: Vec<ChannelPeer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPeer {
    pub pubkey: String,
    pub outbound_ppm_value: Vec<OutboundPpmValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundPpmValue {
    pub current_outbound_fee_ppm: u32,
    pub passive_strategy_ppm_percentile: u32,
    pub active_strategy_ppm_percentile: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFees {
    pub channel_peers: Vec<ChannelPeerFees>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPeerFees {
    pub pubkey: String,
    pub suggested_fees: Vec<SuggestedFee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFee {
    pub passive_fee_rate: u32,
    pub active_fee_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub node_stats: NodeStats,
    pub recommendations: Vec<ChannelRecommendation>,
    pub liquidity_values: OutboundLiquidityValue,
    pub suggested_fees: SuggestedFees,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub pubkey: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub fee_rate: u32,
    pub base_fee: u64,
    pub last_update: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub type_: ActionType,
    pub description: String,
    pub priority: Priority,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    FeeAdjustment,
    ChannelCreation,
    ChannelClosure,
    Rebalancing,
    CapacityAdjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    High,
    Medium,
    Low,
} 