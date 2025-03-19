use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambda_runtime::Context;
use serde_json::json;
use tera::Tera;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(func)).await
}

async fn func(event: Request, _: Context) -> Result<Response<Body>, Error> {
    // Initialisation de Tera
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "text/plain")
                .body(Body::from(format!("Erreur template: {}", e)))
                .expect("failed to render error"));
        }
    };

    // Rendu du template
    match tera.render("index.html", &tera::Context::new()) {
        Ok(html) => Ok(Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(Body::from(html))
            .expect("failed to render template")),
        Err(e) => Ok(Response::builder()
            .status(500)
            .header("content-type", "text/plain")
            .body(Body::from(format!("Erreur: {}", e)))
            .expect("failed to render error"))
    }
} 