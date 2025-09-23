use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::search::{SearchQueryBuilder, SearchResult, SearchSortBy};
use zendesk_api_rust::query::SortOrder;
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let api_token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let config = ZendeskConfig::new(subdomain, AuthMethod::api_token(email, api_token));
    let client = ZendeskClient::new(config)?;

    println!("=== Zendesk Advanced Search Examples ===\n");

    // Example 1: Simple text search across all resources
    println!("1. Simple text search for 'password reset':");
    match client.search("password reset").await {
        Ok(response) => {
            println!("Found {} results", response.results.len());
            for (i, result) in response.results.iter().take(3).enumerate() {
                match result {
                    SearchResult::Ticket(ticket) => {
                        println!("  {}. Ticket #{:?}: {}", i + 1, ticket.id, ticket.subject);
                    }
                    SearchResult::User(user) => {
                        println!("  {}. User: {} ({})", i + 1, user.name, user.email);
                    }
                    SearchResult::Organization(org) => {
                        println!("  {}. Organization: {}", i + 1, org.name);
                    }
                    SearchResult::Group(group) => {
                        println!("  {}. Group: {}", i + 1, group.name);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: Search only tickets with specific criteria
    println!("\n2. Search open tickets assigned to user ID 123:");
    match client
        .search_tickets_advanced("status:open assignee:123")
        .await
    {
        Ok(response) => {
            println!(
                "Found {} open tickets assigned to user 123",
                response.results.len()
            );
            for result in &response.results {
                if let SearchResult::Ticket(ticket) = result {
                    println!(
                        "  - Ticket #{:?}: {} (Status: {:?})",
                        ticket.id, ticket.subject, ticket.status
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Advanced search using query builder
    println!("\n3. Advanced search using query builder:");
    let query = SearchQueryBuilder::new()
        .tickets()
        .status("new")
        .priority("urgent")
        .created_after("2024-01-01")
        .tags("bug")
        .subject_contains("API")
        .build();

    println!("Query: {}", query);

    match client
        .search_advanced_with_sort(
            SearchQueryBuilder::new()
                .tickets()
                .status("new")
                .priority("urgent")
                .created_after("2024-01-01"),
            SearchSortBy::CreatedAt,
            SortOrder::Desc,
        )
        .await
    {
        Ok(response) => {
            println!(
                "Found {} urgent new tickets created after 2024-01-01",
                response.results.len()
            );
            for result in &response.results {
                if let SearchResult::Ticket(ticket) = result {
                    println!(
                        "  - Ticket #{:?}: {} (Created: {:?})",
                        ticket.id, ticket.subject, ticket.created_at
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 4: Search with date ranges
    println!("\n4. Search tickets updated in the last 7 days:");
    let last_week = chrono::Utc::now() - chrono::Duration::days(7);
    let date_str = last_week.format("%Y-%m-%d").to_string();

    let query = SearchQueryBuilder::new()
        .tickets()
        .updated_after(&date_str)
        .build();

    match client.search(&query).await {
        Ok(response) => {
            println!(
                "Found {} tickets updated since {}",
                response.results.len(),
                date_str
            );
            if let Some(next_page) = &response.next_page {
                println!("Has more results available at: {}", next_page);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 5: Search users by email domain
    println!("\n5. Search users with @company.com email addresses:");
    match client.search_users_advanced("email:*@company.com").await {
        Ok(response) => {
            println!(
                "Found {} users with @company.com emails",
                response.results.len()
            );
            for result in response.results.iter().take(5) {
                if let SearchResult::User(user) = result {
                    println!("  - {}: {}", user.name, user.email);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 6: Search organizations
    println!("\n6. Search organizations by name:");
    match client
        .search_organizations_advanced("Acme Corporation")
        .await
    {
        Ok(response) => {
            println!(
                "Found {} organizations matching 'Acme Corporation'",
                response.results.len()
            );
            for result in &response.results {
                if let SearchResult::Organization(org) = result {
                    println!("  - Organization: {} (ID: {:?})", org.name, org.id);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 7: Get search result count without fetching all results
    println!("\n7. Count tickets with 'billing' in subject:");
    match client.search_count("type:ticket subject:billing").await {
        Ok(count) => {
            println!("Total tickets with 'billing' in subject: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 8: Search with custom fields
    println!("\n8. Search tickets with specific custom field value:");
    let custom_field_query = SearchQueryBuilder::new()
        .tickets()
        .custom_field(123456, "VIP") // Replace with actual custom field ID
        .build();

    match client.search(&custom_field_query).await {
        Ok(response) => {
            println!("Found {} VIP tickets", response.results.len());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 9: Complex multi-criteria search
    println!("\n9. Complex search: High priority tickets from last month:");
    let last_month = chrono::Utc::now() - chrono::Duration::days(30);
    let month_ago = last_month.format("%Y-%m-%d").to_string();

    let complex_query = SearchQueryBuilder::new()
        .tickets()
        .priority("high")
        .status("open")
        .created_after(&month_ago)
        .text("urgent")
        .build();

    println!("Complex query: {}", complex_query);

    match client
        .search_with_sort(&complex_query, SearchSortBy::Priority, SortOrder::Desc)
        .await
    {
        Ok(response) => {
            println!(
                "Found {} high priority tickets from last month",
                response.results.len()
            );
            for result in response.results.iter().take(3) {
                if let SearchResult::Ticket(ticket) = result {
                    println!(
                        "  - #{:?}: {} (Priority: {:?}, Status: {:?})",
                        ticket.id, ticket.subject, ticket.priority, ticket.status
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 10: Demonstrate pagination
    println!("\n10. Pagination example:");
    match client.search("type:ticket").await {
        Ok(response) => {
            println!("First page: {} results", response.results.len());

            if let Some(next_page_url) = &response.next_page {
                println!("Fetching next page...");
                match client.search_with_pagination(next_page_url).await {
                    Ok(next_response) => {
                        println!("Second page: {} results", next_response.results.len());
                    }
                    Err(e) => eprintln!("Error fetching next page: {}", e),
                }
            } else {
                println!("No more pages available");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Search Examples Complete ===");

    Ok(())
}
