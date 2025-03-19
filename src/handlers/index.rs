use actix_web::{web, HttpResponse, Responder};
use tera::Tera;
use log::{error, info};
use std::path::Path;

pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Rendu de la page d'accueil");
    
    // Vérification du fichier template
    let template_path = Path::new("templates/index.html");
    if !template_path.exists() {
        error!("Le fichier template index.html n'existe pas!");
        return HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("Le fichier template index.html n'existe pas");
    }
    info!("Fichier template trouvé: {:?}", template_path);

    let ctx = tera::Context::new();
    
    match tmpl.render("index.html", &ctx) {
        Ok(html) => {
            info!("Page d'accueil rendue avec succès");
            HttpResponse::Ok()
                .content_type("text/html")
                .body(html)
        },
        Err(e) => {
            error!("Erreur lors du rendu du template: {}", e);
            error!("Détails de l'erreur: {:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("Erreur interne du serveur: {}\nDétails: {:?}", e, e))
        }
    }
}

pub async fn channels(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Rendu de la page des canaux");
    let ctx = tera::Context::new();
    
    match tmpl.render("channels.html", &ctx) {
        Ok(html) => {
            info!("Page des canaux rendue avec succès");
            HttpResponse::Ok()
                .content_type("text/html")
                .body(html)
        },
        Err(e) => {
            error!("Erreur lors du rendu du template: {}", e);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("Erreur interne du serveur: {}", e))
        }
    }
}

pub async fn transactions(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Rendu de la page des transactions");
    let ctx = tera::Context::new();
    
    match tmpl.render("transactions.html", &ctx) {
        Ok(html) => {
            info!("Page des transactions rendue avec succès");
            HttpResponse::Ok()
                .content_type("text/html")
                .body(html)
        },
        Err(e) => {
            error!("Erreur lors du rendu du template: {}", e);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body(format!("Erreur interne du serveur: {}", e))
        }
    }
} 