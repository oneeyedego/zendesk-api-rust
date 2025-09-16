use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration - using your credentials
    let subdomain = "mit-39553";
    let email = "zendesk.path718@passmail.com";
    let token = "1grbHhsHsS3LnUjllpmt98GX1oK8tnAwHclkFLZb";

    let auth = AuthMethod::api_token(email, token);
    let config = ZendeskConfig::new(subdomain, auth);
    let client = ZendeskClient::new(config)?;

    println!("Fetching Users Examples\n");

    // 1. Get current user info
    println!("1. Getting current user information...");
    match client.get::<serde_json::Value>("users/me.json").await {
        Ok(user_data) => {
            if let Some(user) = user_data.get("user") {
                println!("Current User Details:");
                if let Some(name) = user.get("name").and_then(|n| n.as_str()) {
                    println!("   Name: {}", name);
                }
                if let Some(email) = user.get("email").and_then(|e| e.as_str()) {
                    println!("   Email: {}", email);
                }
                if let Some(role) = user.get("role").and_then(|r| r.as_str()) {
                    println!("   Role: {}", role);
                }
                if let Some(active) = user.get("active").and_then(|a| a.as_bool()) {
                    println!("   Active: {}", active);
                }
                if let Some(created_at) = user.get("created_at").and_then(|c| c.as_str()) {
                    println!("   Created: {}", created_at);
                }
            }
        }
        Err(e) => println!("Failed to get current user: {}", e),
    }

    // 2. List all users
    println!("\n2. Listing all users...");
    match client.list_users().await {
        Ok(users) => {
            println!("Found {} users", users.len());
            for (i, user) in users.iter().take(5).enumerate() {
                println!(
                    "   {}. {} <{}> (ID: {}, Role: {:?})",
                    i + 1,
                    user.name,
                    &user.email,
                    user.id.unwrap_or(0),
                    user.role
                );
            }
            if users.len() > 5 {
                println!("   ... and {} more users", users.len() - 5);
            }

            // Store first user ID for detailed lookup
            if let Some(first_user) = users.first() {
                if let Some(user_id) = first_user.id {
                    println!("\n3. Fetching detailed info for user ID {}...", user_id);
                    match client.get_user(user_id).await {
                        Ok(user_detail) => {
                            println!("User Details:");
                            println!("   Name: {}", user_detail.name);
                            println!("   Email: {}", user_detail.email);
                            println!("   Role: {:?}", user_detail.role);
                            if let Some(phone) = &user_detail.phone {
                                println!("   Phone: {}", phone);
                            }
                            if let Some(timezone) = &user_detail.time_zone {
                                println!("   Timezone: {}", timezone);
                            }
                            if let Some(locale) = &user_detail.locale {
                                println!("   Locale: {}", locale);
                            }
                            println!("   Active: {}", user_detail.active.unwrap_or(false));
                            if let Some(tags) = &user_detail.tags {
                                println!("   Tags: {:?}", tags);
                            }
                        }
                        Err(e) => println!("Failed to fetch user details: {}", e),
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to list users: {}", e);
        }
    }

    // 4. Search users by email domain
    println!("\n4. Searching users by email domain '@passmail.com'...");
    match client.search_users("@passmail.com").await {
        Ok(search_results) => {
            println!(
                "Found {} users with @passmail.com emails",
                search_results.len()
            );
            for user in search_results.iter().take(3) {
                println!(
                    "   - {} <{}> (Role: {:?})",
                    user.name, &user.email, user.role
                );
            }
        }
        Err(e) => {
            println!("Search failed: {}", e);
        }
    }

    // 5. Search for specific user by email
    println!("\n5. Looking up user by email...");
    match client
        .get_user_by_email("zendesk.path718@passmail.com")
        .await
    {
        Ok(user) => {
            println!("Found user by email:");
            println!("   Name: {}", user.name);
            println!("   ID: {}", user.id.unwrap_or(0));
            println!("   Role: {:?}", user.role);
            if let Some(created_at) = &user.created_at {
                println!("   Created: {}", created_at);
            }
        }
        Err(e) => {
            println!("Failed to find user by email: {}", e);
        }
    }

    // 6. Get users in organization (if any exist)
    println!("\n6. Checking for users in organizations...");
    match client.get::<serde_json::Value>("organizations.json").await {
        Ok(orgs_data) => {
            if let Some(orgs) = orgs_data.get("organizations").and_then(|o| o.as_array()) {
                if let Some(first_org) = orgs.first() {
                    if let Some(org_id) = first_org.get("id").and_then(|id| id.as_u64()) {
                        match client.list_users_in_organization(org_id).await {
                            Ok(org_users) => {
                                println!(
                                    "Found {} users in organization {}",
                                    org_users.len(),
                                    org_id
                                );
                                for user in org_users.iter().take(3) {
                                    println!("   - {} <{}>", user.name, &user.email);
                                }
                            }
                            Err(e) => println!("Failed to fetch organization users: {}", e),
                        }
                    }
                } else {
                    println!("No organizations found");
                }
            }
        }
        Err(e) => println!("Failed to fetch organizations: {}", e),
    }

    println!("\nUser fetching examples completed!");

    Ok(())
}
