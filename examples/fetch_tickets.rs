use std::env;
use zendesk_api_rust::auth::AuthMethod;
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

    println!("Fetching Tickets Examples\n");

    // 1. List all tickets
    println!("1. Listing all tickets...");
    match client.list_tickets().await {
        Ok(tickets) => {
            println!("Found {} tickets", tickets.len());
            for (i, ticket) in tickets.iter().take(5).enumerate() {
                println!(
                    "   {}. {} (ID: {}, Status: {:?})",
                    i + 1,
                    ticket.subject,
                    ticket.id.unwrap_or(0),
                    ticket.status
                );
            }
            if tickets.len() > 5 {
                println!("   ... and {} more tickets", tickets.len() - 5);
            }

            // Store first ticket ID for further examples
            if let Some(first_ticket) = tickets.first() {
                if let Some(ticket_id) = first_ticket.id {
                    println!("\n2. Fetching detailed info for ticket ID {}...", ticket_id);
                    match client.get_ticket(ticket_id).await {
                        Ok(ticket_detail) => {
                            println!("Ticket Details:");
                            println!("   Subject: {}", ticket_detail.subject);
                            if let Some(desc) = &ticket_detail.description {
                                let short_desc = if desc.len() > 100 {
                                    format!("{}...", &desc[..100])
                                } else {
                                    desc.clone()
                                };
                                println!("   Description: {}", short_desc);
                            }
                            println!("   Status: {:?}", ticket_detail.status);
                            println!("   Priority: {:?}", ticket_detail.priority);
                            if let Some(tags) = &ticket_detail.tags {
                                println!("   Tags: {:?}", tags);
                            }
                        }
                        Err(e) => println!("Failed to fetch ticket details: {}", e),
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to list tickets: {}", e);
        }
    }

    // 3. Search tickets by keyword
    println!("\n3. Searching tickets with keyword 'test'...");
    match client.search_tickets("test").await {
        Ok(search_results) => {
            println!("Found {} tickets matching 'test'", search_results.len());
            for ticket in search_results.iter().take(3) {
                println!("   - {} (ID: {})", ticket.subject, ticket.id.unwrap_or(0));
            }
        }
        Err(e) => {
            println!("Search failed: {}", e);
        }
    }

    // 4. Get current user's assigned tickets
    println!("\n4. Fetching your assigned tickets...");
    // First get current user ID
    match client.get::<serde_json::Value>("users/me.json").await {
        Ok(user_data) => {
            if let Some(user_id) = user_data
                .get("user")
                .and_then(|u| u.get("id"))
                .and_then(|id| id.as_u64())
            {
                match client.list_tickets_assigned_to(user_id).await {
                    Ok(assigned_tickets) => {
                        println!("You have {} assigned tickets", assigned_tickets.len());
                        for ticket in assigned_tickets.iter().take(3) {
                            println!("   - {} (Priority: {:?})", ticket.subject, ticket.priority);
                        }
                    }
                    Err(e) => println!("Failed to fetch assigned tickets: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to get current user: {}", e),
    }

    println!("\nTicket fetching examples completed!");

    Ok(())
}
