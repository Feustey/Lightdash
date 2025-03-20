#[derive(Clone, Deserialize)]
pub struct NodeStats {
    pub pubkey: String,
    pub alias: String,
    pub capacity: f64,
    pub total_channels: u32,
    pub fee_rates: Vec<f64>,
    pub routing_fees: Vec<f64>,
    pub routing_volume: Vec<f64>,
    pub last_update: String,
    pub uptime: f64,
    pub avg_fee_rate: f64,
    pub total_routing_fees: f64,
    pub total_routing_volume: f64,
} 