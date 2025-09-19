use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::{SearchQueryBuilder, SearchResult};
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let api_token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let config = ZendeskConfig::new(subdomain, AuthMethod::api_token(email, api_token));
    let client = ZendeskClient::new(config)?;

    println!("=== Basic Zendesk Search Examples ===\n");

    // Example 1: Search for tickets containing "bug"
    println!("1. Searching for tickets containing 'bug':");
    let query = SearchQueryBuilder::new().tickets().text("bug").build();

    match client.search(&query).await {
        Ok(response) => {
            println!("Found {} tickets containing 'bug'", response.results.len());
            for result in response.results.iter().take(5) {
                if let SearchResult::Ticket(ticket) = result {
                    println!("  - #{:?}: {}", ticket.id, ticket.subject);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: Search for open tickets
    println!("\n2. Searching for open tickets:");
    let query = SearchQueryBuilder::new().tickets().status("open").build();

    match client.search(&query).await {
        Ok(response) => {
            println!("Found {} open tickets", response.results.len());
            for result in response.results.iter().take(3) {
                if let SearchResult::Ticket(ticket) = result {
                    println!(
                        "  - #{:?}: {} (Status: {:?})",
                        ticket.id, ticket.subject, ticket.status
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Search for high priority tickets
    println!("\n3. Searching for high priority tickets:");
    let query = SearchQueryBuilder::new().tickets().priority("high").build();

    match client.search(&query).await {
        Ok(response) => {
            println!("Found {} high priority tickets", response.results.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 4: Count results before fetching
    println!("\n4. Getting count of urgent tickets:");
    let query = SearchQueryBuilder::new()
        .tickets()
        .priority("urgent")
        .build();

    match client.search_count(&query).await {
        Ok(count) => {
            println!("Total urgent tickets: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 5: Simple text search across all resources
    println!("\n5. Simple search for 'password':");
    match client.search("password").await {
        Ok(response) => {
            println!(
                "Found {} total results containing 'password'",
                response.results.len()
            );

            let mut ticket_count = 0;
            let mut user_count = 0;
            let mut org_count = 0;

            for result in &response.results {
                match result {
                    SearchResult::Ticket(_) => ticket_count += 1,
                    SearchResult::User(_) => user_count += 1,
                    SearchResult::Organization(_) => org_count += 1,
                    SearchResult::Group(_) => {}
                }
            }

            println!("  - Tickets: {}", ticket_count);
            println!("  - Users: {}", user_count);
            println!("  - Organizations: {}", org_count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Basic Search Examples Complete ===");

    Ok(())
}
