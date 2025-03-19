use opentelemetry::sdk::trace::{self, Sampler};
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing::subscriber::set_global_default;

/// Configure le système de tracing avec OpenTelemetry et Jaeger
pub fn init_telemetry(app_name: &str) {
    // Configuration du tracer Jaeger
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(app_name)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Échec de l'initialisation du tracer Jaeger");

    // Création des layers de logging
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        app_name.into(),
        std::io::stdout,
    );

    // Configuration du subscriber avec tous les layers
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer));

    // Définition du subscriber global
    set_global_default(subscriber)
        .expect("Échec de l'initialisation du subscriber de tracing");

    // Log de démarrage
    tracing::info!("Système de tracing initialisé avec succès");
}

/// Retourne le middleware de tracing pour Actix-web
pub fn get_tracing_middleware() -> TracingLogger<tracing_bunyan_formatter::JsonStorageLayer> {
    TracingLogger::default()
} 