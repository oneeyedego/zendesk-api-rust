use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

// Helper function to show the last comment as proof it was added
async fn show_last_comment(client: &ZendeskClient, ticket_id: u64) {
    match client.get_ticket_comments(ticket_id).await {
        Ok(comments) => {
            if let Some(last_comment) = comments.last() {
                println!("   Comment: {}", last_comment.body);
                println!("   Public: {:?}", last_comment.public.unwrap_or(true));
            }
        }
        Err(e) => eprintln!("   Could not retrieve comments: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let api_token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let config = ZendeskConfig::new(subdomain, AuthMethod::api_token(email, api_token));
    let client = ZendeskClient::new(config)?;

    let ticket_id = 1; // Replace with actual ticket ID

    println!("=== Simple Comment Examples ===\n");

    // Add a public response (customer can see this)
    println!("Adding public response...");
    match client
        .add_public_response(ticket_id, "Thank you for your inquiry!")
        .await
    {
        Ok(_) => {
            println!("Public response added");
            show_last_comment(&client, ticket_id).await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Add a work note (internal only, customer cannot see this)
    println!("\nAdding work note...");
    match client
        .add_work_note(ticket_id, "Customer seems satisfied with the solution.")
        .await
    {
        Ok(_) => {
            println!("Work note added");
            show_last_comment(&client, ticket_id).await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Solve ticket with a response
    println!("\nSolving ticket with response...");
    match client
        .solve_ticket_with_response(
            ticket_id,
            "Your issue has been resolved. Please let us know if you need further help!",
        )
        .await
    {
        Ok(_) => {
            println!("Ticket solved with response");
            show_last_comment(&client, ticket_id).await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Complete ===");

    Ok(())
}
