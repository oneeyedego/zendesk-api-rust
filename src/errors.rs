use thiserror::Error;

pub type Result<T> = std::result::Result<T, ZendeskError>;

#[derive(Error, Debug)]
pub enum ZendeskError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("Authentication failed: {message}")]
    Auth { message: String },
    
    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },
    
    #[error("Rate limit exceeded: {retry_after:?}")]
    RateLimit { retry_after: Option<u64> },
    
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Validation error: {message}")]
    Validation { message: String },
}

impl ZendeskError {
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth {
            message: message.into(),
        }
    }
    
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
        }
    }
    
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }
    
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}