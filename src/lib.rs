pub mod models;
pub mod services;
pub mod handlers;
pub mod telemetry;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use tera::Tera;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use actix_web::middleware;

use handlers::{index, lightning};
use services::lightning::LightningService;
use telemetry::{init_telemetry, get_tracing_middleware};

pub async fn run_server() -> std::io::Result<()> {
    // Configuration de base
    dotenv().ok();
    
    // Initialisation du tracing
    init_telemetry("lightdash");
    tracing::info!("Démarrage de l'application Lightdash Rust");

    // Récupération du port depuis les variables d'environnement
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_address = format!("{}:{}", host, port);
    tracing::info!(bind_address = %bind_address, "Configuration du serveur");

    // Test du dossier templates
    let templates_path = std::path::Path::new("templates");
    if !templates_path.exists() {
        tracing::error!("Le dossier templates n'existe pas");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Le dossier templates n'existe pas",
        ));
    }
    tracing::info!("Dossier templates trouvé");

    // Test du dossier static
    let static_path = std::path::Path::new("static");
    if !static_path.exists() {
        tracing::error!("Le dossier static n'existe pas");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Le dossier static n'existe pas",
        ));
    }
    tracing::info!("Dossier static trouvé");

    // Configuration de Tera
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            tracing::info!("Template engine initialisé avec succès");
            t
        }
        Err(e) => {
            tracing::error!(error = %e, "Erreur lors de l'initialisation du template engine");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Erreur d'initialisation des templates: {}", e),
            ));
        }
    };
    let tera = Arc::new(tera);

    // Configuration des services externes
    let sparkseer_url = env::var("NEXT_PUBLIC_API_URL")
        .unwrap_or_else(|_| "https://api.sparkseer.space".to_string());
    let ml_url = env::var("NEXT_PUBLIC_1ML_URL")
        .unwrap_or_else(|_| "https://1ml.com".to_string());
    tracing::info!(sparkseer_url = %sparkseer_url, ml_url = %ml_url, "URLs des services externes configurées");

    // Configuration du service Lightning
    let lightning_url = env::var("LIGHTNING_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    tracing::info!(lightning_url = %lightning_url, "URL du service Lightning configurée");
    let lightning_service = web::Data::new(LightningService::new(
        lightning_url,
        sparkseer_url,
        ml_url,
    ));

    // Configuration du serveur
    tracing::info!(bind_address = %bind_address, "Démarrage du serveur");
    HttpServer::new(move || {
        App::new()
            .wrap(get_tracing_middleware())
            .app_data(web::Data::new(tera.clone()))
            .app_data(lightning_service.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/favicon.ico", web::get().to(|| async {
                match std::fs::read("static/favicon.ico") {
                    Ok(content) => HttpResponse::Ok()
                        .content_type("image/x-icon")
                        .body(content),
                    Err(_) => HttpResponse::NotFound().finish()
                }
            }))
            .route("/", web::get().to(index::index))
            .route("/channels", web::get().to(index::channels))
            .route("/transactions", web::get().to(index::transactions))
            .route("/api/node/info", web::get().to(lightning::get_node_info))
            .route("/api/channels", web::get().to(lightning::list_channels))
            .route("/api/transactions", web::get().to(lightning::list_transactions))
            .route("/api/network/stats", web::get().to(lightning::get_network_stats))
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().body("Page non trouvée")
            }))
    })
    .bind(&bind_address)?
    .run()
    .await
} 