use actix_web::{web, HttpResponse, Responder};
use crate::services::lightning::LightningService;
use crate::models::lightning::{Channel, NodeInfo, Transaction};

pub async fn get_node_info(service: web::Data<LightningService>) -> impl Responder {
    match service.get_node_info().await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des informations du nœud: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Erreur lors de la récupération des informations du nœud"
            }))
        }
    }
}

pub async fn list_channels(service: web::Data<LightningService>) -> impl Responder {
    match service.list_channels().await {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des canaux: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Erreur lors de la récupération des canaux"
            }))
        }
    }
}

pub async fn list_transactions(service: web::Data<LightningService>) -> impl Responder {
    match service.list_transactions().await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des transactions: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Erreur lors de la récupération des transactions"
            }))
        }
    }
} 