use crate::auth::AuthMethod;
use crate::errors::{Result, ZendeskError};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZendeskConfig {
    /// Zendesk subdomain (e.g., "company" for company.zendesk.com)
    pub subdomain: String,
    
    /// Authentication method
    pub auth: AuthMethod,
    
    /// API version (defaults to "v2")
    pub api_version: String,
    
    /// Request timeout in seconds (defaults to 30)
    pub timeout_seconds: u64,
    
    /// Maximum number of retry attempts (defaults to 3)
    pub max_retries: u32,
    
    /// Custom user agent (optional)
    pub user_agent: Option<String>,
}

impl ZendeskConfig {
    pub fn new(subdomain: impl Into<String>, auth: AuthMethod) -> Self {
        Self {
            subdomain: subdomain.into(),
            auth,
            api_version: "v2".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            user_agent: Some(format!("zendesk-api-rust/{}", env!("CARGO_PKG_VERSION"))),
        }
    }
    
    pub fn with_api_version(mut self, version: impl Into<String>) -> Self {
        self.api_version = version.into();
        self
    }
    
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
    
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    pub fn base_url(&self) -> Result<Url> {
        let url_str = format!("https://{}.zendesk.com/api/{}/", self.subdomain, self.api_version);
        Url::parse(&url_str).map_err(ZendeskError::from)
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.subdomain.is_empty() {
            return Err(ZendeskError::config("Subdomain cannot be empty"));
        }
        
        if self.api_version.is_empty() {
            return Err(ZendeskError::config("API version cannot be empty"));
        }
        
        if self.timeout_seconds == 0 {
            return Err(ZendeskError::config("Timeout must be greater than 0"));
        }
        
        // Validate base URL can be constructed
        self.base_url()?;
        
        Ok(())
    }
}