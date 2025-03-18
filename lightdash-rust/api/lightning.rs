use lambda_http::{handler, lambda_runtime::{self, Context}, IntoResponse, Request, Response};
use serde_json::json;
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, lambda_runtime::Error> {
    // Vérification des variables d'environnement requises
    let lightning_url = env::var("LIGHTNING_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    let macaroon = env::var("LIGHTNING_MACAROON")
        .unwrap_or_default();
    
    let cert = env::var("LIGHTNING_CERT")
        .unwrap_or_default();

    // Configuration du client avec les certificats et le macaroon
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // À utiliser uniquement en développement
        .build()
        .unwrap();

    let path = event.uri().path();
    let response = match path {
        "/api/node/info" => {
            client.get(format!("{}/v1/getinfo", lightning_url))
                .header("Grpc-Metadata-macaroon", &macaroon)
                .send()
                .await
        },
        "/api/channels" => {
            client.get(format!("{}/v1/channels", lightning_url))
                .header("Grpc-Metadata-macaroon", &macaroon)
                .send()
                .await
        },
        "/api/transactions" => {
            client.get(format!("{}/v1/payments", lightning_url))
                .header("Grpc-Metadata-macaroon", &macaroon)
                .send()
                .await
        },
        _ => {
            return Ok(Response::builder()
                .status(404)
                .header("content-type", "application/json")
                .body(json!({"error": "Route non trouvée"}).to_string())
                .expect("failed to render error"))
        }
    };

    match response {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_else(|_| "{}".to_string());
            
            Ok(Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, Grpc-Metadata-macaroon")
                .body(body)
                .expect("failed to render response"))
        },
        Err(e) => {
            Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json!({"error": format!("Erreur: {}", e)}).to_string())
                .expect("failed to render error"))
        }
    }
} 