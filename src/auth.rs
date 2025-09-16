use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Basic authentication with API token
    /// Format: email/token:{api_token}
    ApiToken { email: String, token: String },
    
    /// Basic authentication with password
    /// Format: email:{password}
    Password { email: String, password: String },
    
    /// OAuth bearer token
    Bearer { token: String },
}

impl AuthMethod {
    pub fn api_token(email: impl Into<String>, token: impl Into<String>) -> Self {
        Self::ApiToken {
            email: email.into(),
            token: token.into(),
        }
    }
    
    pub fn password(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Password {
            email: email.into(),
            password: password.into(),
        }
    }
    
    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer {
            token: token.into(),
        }
    }
    
    pub fn to_header_value(&self) -> String {
        match self {
            AuthMethod::ApiToken { email, token } => {
                let credentials = format!("{}/token:{}", email, token);
                let encoded = STANDARD.encode(credentials.as_bytes());
                format!("Basic {}", encoded)
            }
            AuthMethod::Password { email, password } => {
                let credentials = format!("{}:{}", email, password);
                let encoded = STANDARD.encode(credentials.as_bytes());
                format!("Basic {}", encoded)
            }
            AuthMethod::Bearer { token } => {
                format!("Bearer {}", token)
            }
        }
    }
}