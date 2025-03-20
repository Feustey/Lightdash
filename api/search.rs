use lambda_http::{run, service_fn, Body, Error, Request, Response, RequestExt};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
struct NodeInfo {
    pubkey: String,
    alias: String,
    capacity: u64,
    num_channels: u32,
    active_channels: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(func)).await
}

pub async fn func(event: Request) -> Result<Response<Body>, Error> {
    let query = event.query_string_parameters().first("q").unwrap_or("");

    if query.is_empty() {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "error": "Le paramètre de recherche est requis"
                })
                .to_string(),
            ))?);
    }

    let client = Client::new();
    let url = if query.len() == 66 {
        format!("https://1ml.com/node/{}/json", query)
    } else {
        format!("https://1ml.com/node/search?q={}", query)
    };

    match client.get(&url).send().await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                let nodes: Vec<NodeInfo> = response.json().await?;
                Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&nodes)?))?)
            } else {
                Ok(Response::builder()
                    .status(status.as_u16())
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "error": "Erreur lors de la recherche du nœud"
                        })
                        .to_string(),
                    ))?)
            }
        }
        Err(e) => Ok(Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "error": format!("Erreur de requête: {}", e)
                })
                .to_string(),
            ))?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http;

    #[tokio::test]
    async fn test_func_empty_query() {
        let request = Request::builder()
            .uri("https://api.example.com/search")
            .body(Body::Empty)
            .unwrap();
        
        let response = func(request).await.unwrap();
        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_func_valid_query() {
        let request = Request::builder()
            .uri("https://api.example.com/search?q=ACINQ")
            .body(Body::Empty)
            .unwrap();
        
        let response = func(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }
} 