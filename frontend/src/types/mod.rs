use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::ToHtml;
use yew::Html;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeStats {
    pub alias: String,
    pub pubkey: String,
    pub num_channels: u32,
    pub total_capacity: u64,
    pub total_local_balance: u64,
    pub total_remote_balance: u64,
    pub avg_channel_size: u64,
    pub median_channel_size: u64,
    pub num_peers: u32,
    pub num_pending_channels: u32,
    pub uptime_percentage: f64,
}

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
    pub status: ChannelStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Pending,
}

impl fmt::Display for ChannelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelRecommendation {
    pub node_pubkey: String,
    pub node_alias: String,
    pub suggested_size: u64,
    pub confidence_score: f64,
    pub reason: String,
    pub suggested_capacity: u64,
    pub score: f64,
    pub suggested_fees: SuggestedFees,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutboundLiquidityValue {
    pub total_value: u64,
    pub utilization_rate: f64,
    pub value_per_channel: Vec<ChannelValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelValue {
    pub channel_id: String,
    pub remote_pubkey: String,
    pub remote_alias: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub value_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedFees {
    pub base_fee: u64,
    pub fee_rate: u64,
    pub base_fee_msat: u64,
    pub fee_rate_ppm: u64,
    pub time_lock_delta: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub type_: ActionType,
    pub status: ActionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub action_type: String,
    pub description: String,
    pub priority: String,
    pub impact: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    OpenChannel,
    CloseChannel,
    UpdateFees,
    Rebalance,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl fmt::Display for ActionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub num_channels: u32,
    pub total_capacity: u64,
    pub total_local_balance: u64,
    pub total_remote_balance: u64,
    pub num_peers: u32,
    pub uptime_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub type_: TransactionType,
    pub amount: u64,
    pub status: TransactionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Payment,
    Invoice,
    ChannelOpen,
    ChannelClose,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YieldStats {
    pub total_yield: f64,
    pub average_yield: f64,
    pub num_transactions: u32,
    pub period: String,
}

impl ToHtml for ChannelStatus {
    fn to_html(&self) -> Html {
        html! { <p>{self.to_string()}</p> }
    }
}

impl ToHtml for ActionType {
    fn to_html(&self) -> Html {
        html! { <p>{self.to_string()}</p> }
    }
}

impl ToHtml for ActionStatus {
    fn to_html(&self) -> Html {
        html! { <p>{self.to_string()}</p> }
    }
}

impl ToHtml for TransactionType {
    fn to_html(&self) -> Html {
        html! { <p>{self.to_string()}</p> }
    }
}

impl ToHtml for TransactionStatus {
    fn to_html(&self) -> Html {
        html! { <p>{self.to_string()}</p> }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Dashboard,
    Channels,
    Actions,
    Recommendations,
    Yields,
    Alby,
    NotFound,
}

impl Route {
    pub fn to_path(&self) -> String {
        match self {
            Route::Dashboard => "/".to_string(),
            Route::Channels => "/channels".to_string(),
            Route::Actions => "/actions".to_string(),
            Route::Recommendations => "/recommendations".to_string(),
            Route::Yields => "/yields".to_string(),
            Route::Alby => "/alby".to_string(),
            Route::NotFound => "/404".to_string(),
        }
    }
}

impl From<String> for Route {
    fn from(path: String) -> Self {
        match path.as_str() {
            "/" => Route::Dashboard,
            "/channels" => Route::Channels,
            "/actions" => Route::Actions,
            "/recommendations" => Route::Recommendations,
            "/yields" => Route::Yields,
            "/alby" => Route::Alby,
            _ => Route::NotFound,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub code: Option<String>,
} 