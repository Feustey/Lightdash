use lambda_http::{handler, lambda_runtime::{self, Context}, IntoResponse, Request, Response};
use serde_json::json;
use tera::Tera;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, lambda_runtime::Error> {
    // Initialisation de Tera
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "text/plain")
                .body(format!("Erreur template: {}", e))
                .expect("failed to render error"))
        }
    };

    let mut ctx = tera::Context::new();
    
    match tera.render("index.html", &ctx) {
        Ok(html) => Ok(Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(html)
            .expect("failed to render template")),
        Err(e) => Ok(Response::builder()
            .status(500)
            .header("content-type", "text/plain")
            .body(format!("Erreur: {}", e))
            .expect("failed to render error"))
    }
} 