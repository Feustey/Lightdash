use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambda_runtime::Context;
use serde_json::{json, Value};
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(func)).await
}

async fn func(event: Request, _: Context) -> Result<Response<Body>, Error> {
    // Vérification des variables d'environnement requises
    let lightning_url = env::var("LIGHTNING_URL")
        .map_err(|_| "LIGHTNING_URL non définie")?;
    
    let macaroon = env::var("LIGHTNING_MACAROON")
        .map_err(|_| "LIGHTNING_MACAROON non définie")?;
    
    let cert = env::var("LIGHTNING_CERT")
        .map_err(|_| "LIGHTNING_CERT non définie")?;

    // Extraction du chemin de la requête
    let path = event.uri().path();
    
    // Routage des requêtes
    match path {
        "/api/node/info" => {
            let client = Client::new();
            let response = client
                .get(format!("{}/v1/getinfo", lightning_url))
                .header("Grpc-Metadata-macaroon", macaroon)
                .send()
                .await
                .map_err(|e| format!("Erreur de requête: {}", e))?;

            let status = response.status();
            let body = response
                .text()
                .await
                .map_err(|e| format!("Erreur de lecture: {}", e))?;

            Ok(Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(body))
                .expect("failed to render response"))
        }
        "/api/channels" => {
            let client = Client::new();
            let response = client
                .get(format!("{}/v1/channels", lightning_url))
                .header("Grpc-Metadata-macaroon", macaroon)
                .send()
                .await
                .map_err(|e| format!("Erreur de requête: {}", e))?;

            let status = response.status();
            let body = response
                .text()
                .await
                .map_err(|e| format!("Erreur de lecture: {}", e))?;

            Ok(Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(body))
                .expect("failed to render response"))
        },
        "/api/transactions" => {
            let client = Client::new();
            let response = client
                .get(format!("{}/v1/payments", lightning_url))
                .header("Grpc-Metadata-macaroon", macaroon)
                .send()
                .await
                .map_err(|e| format!("Erreur de requête: {}", e))?;

            let status = response.status();
            let body = response
                .text()
                .await
                .map_err(|e| format!("Erreur de lecture: {}", e))?;

            Ok(Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(body))
                .expect("failed to render response"))
        },
        _ => Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(Body::from(json!({"error": "Route non trouvée"}).to_string()))
            .expect("failed to render error"))
    }
} 