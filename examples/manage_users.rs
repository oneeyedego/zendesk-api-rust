use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::user::{User, UserRole};
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

    println!("Managing User Accounts Examples\n");

    // 1. Create a new end user
    println!("1. Creating a new end user...");
    let new_user = User::builder("Test User API Demo", "testuser.api.demo@example.com")
        .role(UserRole::EndUser)
        .phone("+1-555-123-4567")
        .notes("This is a test user created via the Zendesk API for demonstration purposes.")
        .tags(vec![
            "api_test".to_string(),
            "demo_user".to_string(),
            "created_via_api".to_string(),
        ])
        .time_zone("America/New_York")
        .locale("en-US")
        .build();

    let created_user_id = match client.create_user(new_user).await {
        Ok(created_user) => {
            println!("Successfully created user!");
            println!("   ID: {}", created_user.id.unwrap_or(0));
            println!("   Name: {}", created_user.name);
            println!("   Email: {}", created_user.email);
            println!("   Role: {:?}", created_user.role);
            if let Some(phone) = &created_user.phone {
                println!("   Phone: {}", phone);
            }
            if let Some(timezone) = &created_user.time_zone {
                println!("   Timezone: {}", timezone);
            }
            if let Some(tags) = &created_user.tags {
                println!("   Tags: {:?}", tags);
            }
            created_user.id.unwrap_or(0)
        }
        Err(e) => {
            println!("Failed to create user: {}", e);
            return Err(e.into());
        }
    };

    // 2. Update the user's information
    println!("\n2. Updating user information...");
    let user_update = User::builder("Test User API Demo (Updated)", "testuser.api.demo@example.com")
        .role(UserRole::EndUser)
        .phone("+1-555-987-6543")  // Changed phone number
        .notes("This user's information has been updated via the API. Phone number changed and additional tags added.")
        .tags(vec![
            "api_test".to_string(),
            "demo_user".to_string(),
            "created_via_api".to_string(),
            "updated_via_api".to_string(),
            "phone_updated".to_string()
        ])
        .time_zone("America/Los_Angeles")  // Changed timezone
        .locale("en-US")
        .build();

    match client.update_user(created_user_id, user_update).await {
        Ok(updated_user) => {
            println!("Successfully updated user!");
            println!("   ID: {}", updated_user.id.unwrap_or(0));
            println!("   Name: {} (updated)", updated_user.name);
            println!("   Email: {}", updated_user.email);
            if let Some(phone) = &updated_user.phone {
                println!("   Phone: {} (updated)", phone);
            }
            if let Some(timezone) = &updated_user.time_zone {
                println!("   Timezone: {} (updated)", timezone);
            }
            if let Some(tags) = &updated_user.tags {
                println!("   Tags: {:?} (updated)", tags);
            }
        }
        Err(e) => {
            println!("Failed to update user: {}", e);
        }
    }

    // 3. Fetch the user to verify changes
    println!("\n3. Fetching user details to verify updates...");
    match client.get_user(created_user_id).await {
        Ok(fetched_user) => {
            println!("Successfully fetched user details:");
            println!("   ID: {}", fetched_user.id.unwrap_or(0));
            println!("   Name: {}", fetched_user.name);
            println!("   Email: {}", fetched_user.email);
            println!("   Role: {:?}", fetched_user.role);
            println!("   Active: {}", fetched_user.active.unwrap_or(false));
            println!("   Verified: {}", fetched_user.verified.unwrap_or(false));
            if let Some(phone) = &fetched_user.phone {
                println!("   Phone: {}", phone);
            }
            if let Some(notes) = &fetched_user.notes {
                let short_notes = if notes.len() > 100 {
                    format!("{}...", &notes[..100])
                } else {
                    notes.clone()
                };
                println!("   Notes: {}", short_notes);
            }
            if let Some(timezone) = &fetched_user.time_zone {
                println!("   Timezone: {}", timezone);
            }
            if let Some(locale) = &fetched_user.locale {
                println!("   Locale: {}", locale);
            }
            if let Some(tags) = &fetched_user.tags {
                println!("   Tags: {:?}", tags);
            }
            if let Some(created_at) = &fetched_user.created_at {
                println!("   Created: {}", created_at);
            }
            if let Some(updated_at) = &fetched_user.updated_at {
                println!("   Updated: {}", updated_at);
            }
        }
        Err(e) => {
            println!("Failed to fetch user: {}", e);
        }
    }

    // 4. Search for the user by email
    println!("\n4. Searching for user by email...");
    match client
        .get_user_by_email("testuser.api.demo@example.com")
        .await
    {
        Ok(found_user) => {
            println!("Found user by email search:");
            println!("   ID: {}", found_user.id.unwrap_or(0));
            println!("   Name: {}", found_user.name);
            println!("   Email: {}", found_user.email);
            println!("   Role: {:?}", found_user.role);
        }
        Err(e) => {
            println!("Failed to find user by email: {}", e);
        }
    }

    // 5. Create a second user to demonstrate bulk operations
    println!("\n5. Creating a second demo user for bulk operations...");
    let second_user = User::builder("Demo User Two", "demouser2.api.test@example.com")
        .role(UserRole::EndUser)
        .phone("+1-555-111-2222")
        .notes("Second demo user for bulk operations testing.")
        .tags(vec![
            "api_test".to_string(),
            "demo_user".to_string(),
            "bulk_demo".to_string(),
        ])
        .time_zone("America/Chicago")
        .locale("en-US")
        .build();

    let second_user_id = match client.create_user(second_user).await {
        Ok(created_user) => {
            println!("Successfully created second user!");
            println!("   ID: {}", created_user.id.unwrap_or(0));
            println!("   Name: {}", created_user.name);
            println!("   Email: {}", created_user.email);
            created_user.id.unwrap_or(0)
        }
        Err(e) => {
            println!("Failed to create second user: {}", e);
            return Err(e.into());
        }
    };

    // 6. Search for all demo users
    println!("\n6. Searching for all demo users...");
    match client.search_users("demo").await {
        Ok(demo_users) => {
            let api_demo_users: Vec<_> = demo_users
                .iter()
                .filter(|user| user.name.contains("Demo") || user.email.contains("demo"))
                .collect();

            println!("Found {} demo users:", api_demo_users.len());
            for (i, user) in api_demo_users.iter().enumerate() {
                println!(
                    "   {}. {} <{}> (ID: {}, Role: {:?})",
                    i + 1,
                    user.name,
                    user.email,
                    user.id.unwrap_or(0),
                    user.role
                );
            }
        }
        Err(e) => {
            println!("Failed to search for demo users: {}", e);
        }
    }

    // 7. Get user statistics
    println!("\n7. Analyzing user account statistics...");
    match client.list_users().await {
        Ok(all_users) => {
            println!("User Account Statistics:");
            println!("   Total users: {}", all_users.len());

            let active_users = all_users
                .iter()
                .filter(|user| user.active.unwrap_or(false))
                .count();
            println!("   Active users: {}", active_users);

            let verified_users = all_users
                .iter()
                .filter(|user| user.verified.unwrap_or(false))
                .count();
            println!("   Verified users: {}", verified_users);

            let agents = all_users
                .iter()
                .filter(|user| matches!(user.role, Some(UserRole::Agent)))
                .count();
            println!("   Agents: {}", agents);

            let admins = all_users
                .iter()
                .filter(|user| matches!(user.role, Some(UserRole::Admin)))
                .count();
            println!("   Admins: {}", admins);

            let end_users = all_users
                .iter()
                .filter(|user| matches!(user.role, Some(UserRole::EndUser)))
                .count();
            println!("   End Users: {}", end_users);

            // Count users with phone numbers
            let users_with_phone = all_users.iter().filter(|user| user.phone.is_some()).count();
            println!("   Users with phone numbers: {}", users_with_phone);

            // Count unique timezones
            let mut timezones: Vec<_> = all_users
                .iter()
                .filter_map(|user| user.time_zone.as_ref())
                .collect();
            timezones.sort();
            timezones.dedup();
            println!("   Unique timezones: {}", timezones.len());
        }
        Err(e) => {
            println!("Failed to get user statistics: {}", e);
        }
    }

    // 8. Cleanup - Note about deletion
    println!("\n8. User Management Cleanup Notes");
    println!(
        "Created demo users with IDs: {} and {}",
        created_user_id, second_user_id
    );
    println!("User deletion requires special permissions in many Zendesk instances.");
    println!("Demo users will remain active until manually removed by an admin.");
    println!("To delete users, you would use: client.delete_user(user_id)");
    println!("Consider deactivating users instead of deleting them to preserve ticket history.");

    // Note: In a real scenario, you'd set active: false, but our model doesn't expose that field in the create/update struct
    println!("   (In production, you would set the 'active' field to false to deactivate)");

    println!("\nUser management examples completed!");
    println!("\nSummary of operations performed:");
    println!("   - Created new end user with full profile");
    println!("   - Updated user information (name, phone, timezone, tags)");
    println!("   - Fetched user details to verify changes");
    println!("   - Searched for user by email address");
    println!("   - Created second user for bulk operations");
    println!("   - Performed bulk search across all demo users");
    println!("   - Generated comprehensive user statistics");
    println!("   - Demonstrated deactivation approach");

    Ok(())
}
