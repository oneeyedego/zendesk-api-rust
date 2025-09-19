use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::{TicketCommentCreate, TicketStatus};
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

// Helper function to show the last comment as proof it was added
async fn show_last_comment(client: &ZendeskClient, ticket_id: u64, comment_type: &str) {
    match client.get_ticket_comments(ticket_id).await {
        Ok(comments) => {
            if let Some(last_comment) = comments.last() {
                println!("   Last comment ({}): {}", comment_type, last_comment.body);
                println!("   Public: {:?}", last_comment.public.unwrap_or(true));
                println!("   Created: {:?}", last_comment.created_at);
            } else {
                println!("   No comments found");
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

    // Example ticket ID - replace with an actual ticket ID from your Zendesk
    let ticket_id = 1;

    println!("=== Zendesk Comment Posting Examples ===\n");

    // Example 1: Add a public response (visible to customer)
    println!("1. Adding a public response to ticket {}:", ticket_id);
    match client
        .add_public_response(
            ticket_id,
            "Thank you for contacting us! We've received your request and are looking into it.",
        )
        .await
    {
        Ok(_ticket) => {
            println!("Public response added successfully!");
            show_last_comment(&client, ticket_id, "public response").await;
        }
        Err(e) => eprintln!("Error adding public response: {}", e),
    }

    // Example 2: Add a work note (internal note, not visible to customer)
    println!("\n2. Adding an internal work note to ticket {}:", ticket_id);
    match client
        .add_work_note(
            ticket_id,
            "Customer called to follow up. Issue seems to be related to network configuration.",
        )
        .await
    {
        Ok(_ticket) => {
            println!("Work note added successfully!");
            show_last_comment(&client, ticket_id, "work note").await;
        }
        Err(e) => eprintln!("Error adding work note: {}", e),
    }

    // Example 3: Using the comment builder for more control
    println!("\n3. Using comment builder to add a response with author:");
    let comment = TicketCommentCreate::builder("I'll escalate this to our technical team.")
        .public_response() // Make it visible to customer
        .author_id(12345) // Replace with actual agent ID
        .build();

    match client
        .add_ticket_comment(
            ticket_id,
            zendesk_api_rust::models::TicketCommentRequest {
                ticket: zendesk_api_rust::models::TicketUpdate {
                    comment,
                    status: None,
                    priority: None,
                    assignee_id: None,
                    group_id: None,
                    tags: None,
                },
            },
        )
        .await
    {
        Ok(_ticket) => {
            println!("Comment with author added successfully!");
            show_last_comment(&client, ticket_id, "authored comment").await;
        }
        Err(e) => eprintln!("Error adding comment: {}", e),
    }

    // Example 4: Add comment and update ticket status simultaneously
    println!("\n4. Solving ticket with a response:");
    match client
        .solve_ticket_with_response(
            ticket_id,
            "Your issue has been resolved! Please let us know if you need any further assistance.",
        )
        .await
    {
        Ok(ticket) => {
            println!("Ticket solved with response!");
            println!("   New status: {:?}", ticket.status);
            show_last_comment(&client, ticket_id, "solution response").await;
        }
        Err(e) => eprintln!("Error solving ticket: {}", e),
    }

    // Example 5: Reassign ticket with a work note
    println!("\n5. Reassigning ticket with internal note:");
    let new_assignee_id = 67890; // Replace with actual agent ID
    match client
        .reassign_ticket_with_note(
            ticket_id,
            new_assignee_id,
            "Reassigning to specialist team for advanced troubleshooting.",
        )
        .await
    {
        Ok(ticket) => {
            println!("Ticket reassigned with note!");
            println!("   New assignee ID: {:?}", ticket.assignee_id);
            show_last_comment(&client, ticket_id, "reassignment note").await;
        }
        Err(e) => eprintln!("Error reassigning ticket: {}", e),
    }

    // Example 6: Add comment with multiple updates (status, assignee, tags)
    println!("\n6. Adding comment with multiple ticket updates:");
    let comment = TicketCommentCreate::work_note(
        "Investigation complete. Issue was caused by misconfigured firewall rules.",
    );

    match client
        .add_comment_with_updates(
            ticket_id,
            comment,
            Some(TicketStatus::Solved),
            Some(12345), // New assignee
            Some(vec![
                "resolved".to_string(),
                "firewall".to_string(),
                "investigation".to_string(),
            ]),
        )
        .await
    {
        Ok(ticket) => {
            println!("Comment added with updates!");
            println!("   Status: {:?}", ticket.status);
            println!("   Assignee: {:?}", ticket.assignee_id);
            println!("   Tags: {:?}", ticket.tags);
            show_last_comment(&client, ticket_id, "investigation note").await;
        }
        Err(e) => eprintln!("Error adding comment with updates: {}", e),
    }

    // Example 7: Using builder pattern for complex comment
    println!("\n7. Creating complex comment with builder pattern:");
    let complex_comment = TicketCommentCreate::builder(
        "Attached are the logs from our investigation. The issue appears to be intermittent.",
    )
    .work_note() // Make it internal
    .author_id(12345) // Set specific author
    .uploads(vec!["upload_token_123".to_string()]) // Add file uploads
    .build_request();

    match client.add_ticket_comment(ticket_id, complex_comment).await {
        Ok(_ticket) => {
            println!("Complex comment added successfully!");
            show_last_comment(&client, ticket_id, "complex comment with uploads").await;
        }
        Err(e) => eprintln!("Error adding complex comment: {}", e),
    }

    println!("\n=== Comment Posting Examples Complete ===");
    println!("\nComment Types Summary:");
    println!("• Public Response: Visible to customer, use for customer communication");
    println!("• Work Note: Internal only, use for agent-to-agent communication");
    println!("• Combined Updates: Add comments while updating ticket properties");

    Ok(())
}
