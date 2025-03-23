use crate::mcp::{LightningMCP, NodeMetrics, ChannelMetrics, ActionRecommendation};
use crate::models::Action;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTIONS: Mutex<Vec<Action>> = Mutex::new(Vec::new());
}

pub struct ActionService {
    mcp: LightningMCP,
}

impl ActionService {
    pub fn new() -> Self {
        Self {
            mcp: LightningMCP::new(),
        }
    }

    pub async fn get_recommendations(&self, node_stats: &NodeMetrics, channels: &[ChannelMetrics]) -> Vec<ActionRecommendation> {
        self.mcp.generate_recommendations(node_stats, channels)
    }

    pub async fn create_action(&self, recommendation: &ActionRecommendation) -> Result<Action, std::io::Error> {
        let action = Action {
            id: Uuid::new_v4().to_string(),
            type_: recommendation.action_type.clone(),
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            description: recommendation.reason.clone(),
            priority: recommendation.priority,
            confidence: recommendation.confidence,
        };

        let mut actions = ACTIONS.lock().unwrap();
        actions.push(action.clone());
        
        Ok(action)
    }

    pub async fn get_actions(&self) -> Result<Vec<Action>, std::io::Error> {
        let actions = ACTIONS.lock().unwrap();
        Ok(actions.clone())
    }

    pub async fn update_action_status(&self, action_id: &str, status: &str) -> Result<(), std::io::Error> {
        let mut actions = ACTIONS.lock().unwrap();
        if let Some(action) = actions.iter_mut().find(|a| a.id == action_id) {
            action.status = status.to_string();
            action.updated_at = Utc::now();
        }
        Ok(())
    }
} 