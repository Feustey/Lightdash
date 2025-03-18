mod models;
mod services;
mod handlers;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use tera::Tera;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use lightdash_rust::run_server;

use handlers::{index, lightning};
use services::lightning::LightningService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
} 