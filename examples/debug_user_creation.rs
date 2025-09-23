use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::user::{User, UserRole};
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let auth = AuthMethod::api_token(&email, &token);
    let config = ZendeskConfig::new(&subdomain, auth);
    let client = ZendeskClient::new(config)?;

    println!("Testing minimal user creation...");

    // Test 1: Minimal user
    let minimal_user = User::builder("Simple Test User", "simple.test@example.com").build();

    match client.create_user(minimal_user).await {
        Ok(user) => {
            println!(
                "SUCCESS: Minimal user created with ID: {}",
                user.id.unwrap_or(0)
            );
        }
        Err(e) => {
            println!("FAILED: Minimal user creation failed: {}", e);
        }
    }

    // Test 2: User with just role
    let role_user = User::builder("Role Test User", "role.test@example.com")
        .role(UserRole::EndUser)
        .build();

    match client.create_user(role_user).await {
        Ok(user) => {
            println!(
                "SUCCESS: Role user created with ID: {}",
                user.id.unwrap_or(0)
            );
        }
        Err(e) => {
            println!("FAILED: Role user creation failed: {}", e);
        }
    }

    // Test 3: User with different phone format
    let phone_user = User::builder("Phone Test User", "phone.test@example.com")
        .role(UserRole::EndUser)
        .phone("+15551234567") // More standard E.164 format
        .build();

    match client.create_user(phone_user).await {
        Ok(user) => {
            println!(
                "SUCCESS: Phone user created with ID: {}",
                user.id.unwrap_or(0)
            );
        }
        Err(e) => {
            println!("FAILED: Phone user creation failed: {}", e);
        }
    }

    Ok(())
}
