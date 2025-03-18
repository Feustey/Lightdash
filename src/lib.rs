pub mod models;
pub mod services;
pub mod handlers;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use tera::Tera;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

use handlers::{index, lightning};
use services::lightning::LightningService;

pub async fn run_server() -> std::io::Result<()> {
    // Configuration de base
    dotenv().ok();
    env_logger::init();
    println!("Démarrage de l'application Lightdash Rust");

    // Test du dossier templates
    let templates_path = std::path::Path::new("templates");
    if !templates_path.exists() {
        eprintln!("Erreur: Le dossier templates n'existe pas");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Le dossier templates n'existe pas",
        ));
    }
    println!("Dossier templates trouvé");

    // Test du dossier static
    let static_path = std::path::Path::new("static");
    if !static_path.exists() {
        eprintln!("Erreur: Le dossier static n'existe pas");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Le dossier static n'existe pas",
        ));
    }
    println!("Dossier static trouvé");

    // Configuration de Tera
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            println!("Template engine initialisé avec succès");
            t
        }
        Err(e) => {
            eprintln!("Erreur lors de l'initialisation du template engine: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Erreur d'initialisation des templates: {}", e),
            ));
        }
    };
    let tera = Arc::new(tera);

    // Configuration du service Lightning
    let lightning_url = env::var("LIGHTNING_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    println!("URL du service Lightning: {}", lightning_url);
    let lightning_service = web::Data::new(LightningService::new(lightning_url));

    // Configuration du serveur
    println!("Démarrage du serveur sur http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(lightning_service.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index::index))
            .route("/channels", web::get().to(index::channels))
            .route("/transactions", web::get().to(index::transactions))
            .route("/api/node/info", web::get().to(lightning::get_node_info))
            .route("/api/channels", web::get().to(lightning::list_channels))
            .route("/api/transactions", web::get().to(lightning::list_transactions))
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().body("Page non trouvée")
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 