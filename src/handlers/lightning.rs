use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use tracing::{instrument, info, error};

use crate::services::lightning::LightningService;
use crate::models::lightning::{Channel, NodeInfo, Transaction};

#[instrument(skip(service))]
pub async fn get_node_info(service: web::Data<LightningService>) -> impl Responder {
    info!("Traitement de la requête get_node_info");
    match service.get_node_info().await {
        Ok(info) => {
            info!(pubkey = %info.pubkey, "Informations du nœud envoyées");
            HttpResponse::Ok().json(info)
        }
        Err(e) => {
            error!(error = %e, "Erreur lors de la récupération des informations du nœud");
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

#[instrument(skip(service))]
pub async fn list_channels(service: web::Data<LightningService>) -> impl Responder {
    info!("Traitement de la requête list_channels");
    match service.list_channels().await {
        Ok(channels) => {
            info!(count = channels.len(), "Liste des canaux envoyée");
            HttpResponse::Ok().json(channels)
        }
        Err(e) => {
            error!(error = %e, "Erreur lors de la récupération des canaux");
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

#[instrument(skip(service))]
pub async fn list_transactions(service: web::Data<LightningService>) -> impl Responder {
    info!("Traitement de la requête list_transactions");
    match service.list_transactions().await {
        Ok(transactions) => {
            info!(count = transactions.len(), "Liste des transactions envoyée");
            HttpResponse::Ok().json(transactions)
        }
        Err(e) => {
            error!(error = %e, "Erreur lors de la récupération des transactions");
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
}

#[instrument(skip(service))]
pub async fn get_network_stats(service: web::Data<LightningService>) -> impl Responder {
    info!("Traitement de la requête get_network_stats");
    match service.get_network_stats().await {
        Ok(stats) => {
            info!("Statistiques réseau envoyées");
            HttpResponse::Ok().json(stats)
        }
        Err(e) => {
            error!(error = %e, "Erreur lors de la récupération des statistiques réseau");
            HttpResponse::InternalServerError().json(format!("Erreur: {}", e))
        }
    }
} 