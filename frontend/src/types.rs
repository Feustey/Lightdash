use yew_router::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)]
pub enum Route { #[allow(dead_code)] Fallback,
    Home,
    Channels,
    Recommendations,
    About,

    Home,
    Actions,
    Alby,
    Recommendations,
    Channels,
    About,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Page {
    Dashboard,
    Actions,
    Alby,
}

impl Page {
    pub fn to_string(&self) -> String {
        match self {
            Page::Dashboard => "dashboard".to_string(),
            Page::Actions => "actions".to_string(),
            Page::Alby => "alby".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Page> {
        match s {
            "dashboard" => Some(Page::Dashboard),
            "actions" => Some(Page::Actions),
            "alby" => Some(Page::Alby),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub pubkey: String,
    pub alias: String,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub total_capacity: u64,
    pub num_channels: u32,
    pub avg_channel_size: u64,
    pub uptime_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub channel_id: String,
    pub remote_pubkey: String,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub capacity: u64,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)]
pub struct OutboundLiquidityValue {
    pub value: f64,
    pub value_per_channel: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedFees {
    pub base_fee_msat: u64,
    pub fee_rate_ppm: u32,
    pub time_lock_delta: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: String,
    pub description: String,
    pub priority: String,
    pub impact: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRecommendation {
    pub pubkey: String,
    pub alias: String,
    pub capacity: u64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Doughnut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
} 
impl Default for NodeStats {
    fn default() -> Self {
        Self {
            alias: String::new(),
            pubkey: String::new(),
            local_balance: 0.0,
            remote_balance: 0.0,
            total_capacity: 0,
            avg_channel_size: 0,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            id: String::new(),
            action_type: String::new(),
            node_alias: String::new(),
            description: String::new(),
            amount: 0.0,
            status: String::new(),
            created_at: String::new(),
        }
    }
}

impl Default for NodeStats {
    fn default() -> Self {
        Self {
            alias: String::new(),
            pubkey: String::new(),
            local_balance: 0.0,
            remote_balance: 0.0,
            total_capacity: 0,
            avg_channel_size: 0,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            id: String::new(),
            action_type: String::new(),
            node_alias: String::new(),
            description: String::new(),
            amount: 0.0,
            status: String::new(),
            created_at: String::new(),
        }
    }
}
