use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let api_token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    let config = ZendeskConfig::new(subdomain, AuthMethod::api_token(email, api_token));
    let client = ZendeskClient::new(config)?;

    // Example ticket ID - replace with an actual ticket ID from your Zendesk
    let ticket_id = 1;

    println!("Fetching ticket comments for ticket ID: {}", ticket_id);

    // Get all comments for the ticket
    match client.get_ticket_comments(ticket_id).await {
        Ok(comments) => {
            println!("Found {} comments:", comments.len());

            for (index, comment) in comments.iter().enumerate() {
                println!("\n--- Comment {} ---", index + 1);
                println!("ID: {:?}", comment.id);
                println!("Author ID: {:?}", comment.author_id);
                println!("Created: {:?}", comment.created_at);
                println!("Public: {:?}", comment.public.unwrap_or(true));
                println!("Body: {}", comment.body);

                // Show if this is a public reply or internal note
                match comment.public {
                    Some(true) => println!("Type: Public Reply (visible to requester)"),
                    Some(false) => println!("Type: Internal Note (agents only)"),
                    None => println!("Type: Unknown visibility"),
                }

                // Show attachments if any
                if let Some(attachments) = &comment.attachments {
                    if !attachments.is_empty() {
                        println!("Attachments:");
                        for attachment in attachments {
                            println!("  - {} ({} bytes)", attachment.file_name, attachment.size);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching comments: {}", e);
        }
    }

    // Get comment count
    println!("\n--- Comment Count ---");
    match client.count_ticket_comments(ticket_id).await {
        Ok(count) => {
            println!("Total comments: {}", count);
        }
        Err(e) => {
            eprintln!("Error getting comment count: {}", e);
        }
    }

    // Example of paginated comment retrieval
    println!("\n--- Paginated Comments ---");
    match client
        .get_ticket_comments_with_pagination(ticket_id, None)
        .await
    {
        Ok(response) => {
            println!("Comments in this page: {}", response.comments.len());
            if let Some(next_page) = &response.next_page {
                println!("Has next page: {}", next_page);
            }
            if let Some(prev_page) = &response.previous_page {
                println!("Has previous page: {}", prev_page);
            }
        }
        Err(e) => {
            eprintln!("Error fetching paginated comments: {}", e);
        }
    }

    // Example of making a comment private (uncomment to test)
    // Note: This requires a valid comment ID and will modify the comment
    /*
    let comment_id = 123; // Replace with actual comment ID
    println!("\n--- Making Comment Private ---");
    match client.make_comment_private(ticket_id, comment_id).await {
        Ok(updated_comment) => {
            println!("Comment {} is now private: {:?}", comment_id, updated_comment.public);
        }
        Err(e) => {
            eprintln!("Error making comment private: {}", e);
        }
    }
    */

    Ok(())
}
