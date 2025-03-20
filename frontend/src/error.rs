use std::fmt;
use wasm_bindgen::JsValue;
use log::error;

/// Types d'erreurs possibles dans l'application
#[derive(Debug)]
pub enum AppError {
    /// Erreur lors d'une requête API
    ApiError {
        status: u16,
        message: String,
    },
    /// Erreur de validation des données
    ValidationError(String),
    /// Erreur lors du traitement des recommandations
    RecommendationError(String),
    /// Erreur lors de l'accès aux clés API
    ApiKeyError(String),
    /// Erreur réseau
    NetworkError(String),
    /// Erreur inattendue
    UnexpectedError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ApiError { status, message } => {
                write!(f, "Erreur API ({}): {}", status, message)
            }
            AppError::ValidationError(msg) => write!(f, "Erreur de validation: {}", msg),
            AppError::RecommendationError(msg) => write!(f, "Erreur de recommandation: {}", msg),
            AppError::ApiKeyError(msg) => write!(f, "Erreur de clé API: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Erreur réseau: {}", msg),
            AppError::UnexpectedError(msg) => write!(f, "Erreur inattendue: {}", msg),
        }
    }
}

impl From<AppError> for JsValue {
    fn from(error: AppError) -> Self {
        error!("{}", error);
        JsValue::from_str(&error.to_string())
    }
}

impl From<gloo_net::Error> for AppError {
    fn from(error: gloo_net::Error) -> Self {
        AppError::NetworkError(error.to_string())
    }
}

/// Fonction utilitaire pour gérer les erreurs API
pub fn handle_api_error(status: u16, message: &str) -> AppError {
    let error = AppError::ApiError {
        status,
        message: message.to_string(),
    };
    error!("{}", error);
    error
}

/// Fonction utilitaire pour gérer les erreurs de validation
pub fn handle_validation_error(message: &str) -> AppError {
    let error = AppError::ValidationError(message.to_string());
    error!("{}", error);
    error
}

/// Fonction utilitaire pour gérer les erreurs de recommandation
pub fn handle_recommendation_error(message: &str) -> AppError {
    let error = AppError::RecommendationError(message.to_string());
    error!("{}", error);
    error
}

/// Fonction utilitaire pour gérer les erreurs de clé API
pub fn handle_api_key_error(message: &str) -> AppError {
    let error = AppError::ApiKeyError(message.to_string());
    error!("{}", error);
    error
}

/// Fonction utilitaire pour gérer les erreurs inattendues
pub fn handle_unexpected_error(message: &str) -> AppError {
    let error = AppError::UnexpectedError(message.to_string());
    error!("{}", error);
    error
} 