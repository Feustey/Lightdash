use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::services::lightning::LightningService;
use crate::models::lightning::{Channel, NodeInfo, Transaction};

pub async fn get_node_info(service: web::Data<LightningService>) -> impl Responder {
    match service.get_node_info().await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des informations du nœud: {}", e);
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

pub async fn list_channels(service: web::Data<LightningService>) -> impl Responder {
    match service.list_channels().await {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des canaux: {}", e);
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

pub async fn list_transactions(service: web::Data<LightningService>) -> impl Responder {
    match service.list_transactions().await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des transactions: {}", e);
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

pub async fn get_network_stats(service: web::Data<LightningService>) -> impl Responder {
    match service.get_network_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des statistiques réseau: {}", e);
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
} 