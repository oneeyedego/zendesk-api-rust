use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration
    let subdomain = "mit-39553";
    let email = "zendesk.path718@passmail.com";
    let token = "1grbHhsHsS3LnUjllpmt98GX1oK8tnAwHclkFLZb";

    // Create authentication method
    let auth = AuthMethod::api_token(email, token);

    // Create configuration
    let config = ZendeskConfig::new(subdomain, auth);

    // Validate configuration
    config.validate()?;
    println!("✓ Configuration is valid");

    // Create client
    let client = ZendeskClient::new(config)?;
    println!("✓ Client created successfully");

    // Test API connection by fetching current user info
    println!("\nTesting API connection...");

    let base_url = client.config().base_url()?;
    println!("Base URL: {}", base_url);

    // Try to get current user information to verify the API works
    match client.get::<serde_json::Value>("users/me.json").await {
        Ok(user_data) => {
            println!("✓ API connection successful!");
            if let Some(user) = user_data.get("user") {
                if let Some(name) = user.get("name").and_then(|n| n.as_str()) {
                    println!("  Authenticated as: {}", name);
                }
                if let Some(email) = user.get("email").and_then(|e| e.as_str()) {
                    println!("  Email: {}", email);
                }
                if let Some(role) = user.get("role").and_then(|r| r.as_str()) {
                    println!("  Role: {}", role);
                }
            }
        }
        Err(e) => {
            println!("❌ API connection failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n✓ Zendesk API client is ready for use!");

    Ok(())
}
