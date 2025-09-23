use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::user::{User, UserRole};
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let auth = AuthMethod::api_token(&email, &token);
    let config = ZendeskConfig::new(&subdomain, auth);
    let client = ZendeskClient::new(config)?;

    // Test adding one field at a time to find the problematic one

    // Test 1: Add notes
    println!("Test 1: Adding notes...");
    let notes_user = User::builder("Notes Test User", "notes.test@example.com")
        .role(UserRole::EndUser)
        .notes("This is a test user created via the Zendesk API for demonstration purposes.")
        .build();

    match client.create_user(notes_user).await {
        Ok(user) => println!(
            "SUCCESS: Notes user created with ID: {}",
            user.id.unwrap_or(0)
        ),
        Err(e) => println!("FAILED: Notes user creation failed: {}", e),
    }

    // Test 2: Add tags
    println!("\nTest 2: Adding tags...");
    let tags_user = User::builder("Tags Test User", "tags.test@example.com")
        .role(UserRole::EndUser)
        .tags(vec![
            "api_test".to_string(),
            "demo_user".to_string(),
            "created_via_api".to_string(),
        ])
        .build();

    match client.create_user(tags_user).await {
        Ok(user) => println!(
            "SUCCESS: Tags user created with ID: {}",
            user.id.unwrap_or(0)
        ),
        Err(e) => println!("FAILED: Tags user creation failed: {}", e),
    }

    // Test 3: Add timezone
    println!("\nTest 3: Adding timezone...");
    let tz_user = User::builder("Timezone Test User", "tz.test@example.com")
        .role(UserRole::EndUser)
        .time_zone("America/New_York")
        .build();

    match client.create_user(tz_user).await {
        Ok(user) => println!(
            "SUCCESS: Timezone user created with ID: {}",
            user.id.unwrap_or(0)
        ),
        Err(e) => println!("FAILED: Timezone user creation failed: {}", e),
    }

    // Test 4: Add locale
    println!("\nTest 4: Adding locale...");
    let locale_user = User::builder("Locale Test User", "locale.test@example.com")
        .role(UserRole::EndUser)
        .locale("en-US")
        .build();

    match client.create_user(locale_user).await {
        Ok(user) => println!(
            "SUCCESS: Locale user created with ID: {}",
            user.id.unwrap_or(0)
        ),
        Err(e) => println!("FAILED: Locale user creation failed: {}", e),
    }

    // Test 5: All together (what was failing)
    println!("\nTest 5: All fields together...");
    let complex_user = User::builder("Complex Test User", "complex.test@example.com")
        .role(UserRole::EndUser)
        .phone("+15551234567")
        .notes("This is a test user created via the Zendesk API for demonstration purposes.")
        .tags(vec![
            "api_test".to_string(),
            "demo_user".to_string(),
            "created_via_api".to_string(),
        ])
        .time_zone("America/New_York")
        .locale("en-US")
        .build();

    match client.create_user(complex_user).await {
        Ok(user) => println!(
            "SUCCESS: Complex user created with ID: {}",
            user.id.unwrap_or(0)
        ),
        Err(e) => println!("FAILED: Complex user creation failed: {}", e),
    }

    Ok(())
}
