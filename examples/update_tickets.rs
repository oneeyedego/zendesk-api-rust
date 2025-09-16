use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::ticket::{Ticket, TicketPriority, TicketStatus, TicketType};
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

    println!("Updating Tickets Examples\n");

    // Get current user ID
    let current_user_id = match client.get::<serde_json::Value>("users/me.json").await {
        Ok(user_data) => user_data
            .get("user")
            .and_then(|u| u.get("id"))
            .and_then(|id| id.as_u64())
            .ok_or("Could not get current user ID")?,
        Err(e) => {
            println!("Failed to get current user: {}", e);
            return Err(e.into());
        }
    };

    // First, create a test ticket to update
    println!("Setting up: Creating a test ticket to update...");
    let test_ticket = Ticket::builder("API Test - Ticket for Update Demo")
        .comment("This ticket will be updated as part of the API demonstration.")
        .priority(TicketPriority::Low)
        .ticket_type(TicketType::Question)
        .status(TicketStatus::New)
        .requester_id(current_user_id)
        .tags(vec!["api_test".to_string(), "update_demo".to_string()])
        .build();

    let ticket_id = match client.create_ticket(test_ticket).await {
        Ok(created_ticket) => {
            let id = created_ticket.id.unwrap_or(0);
            println!("Created test ticket with ID: {}", id);
            id
        }
        Err(e) => {
            println!("Failed to create test ticket: {}", e);
            return Err(e.into());
        }
    };

    // 1. Update ticket priority and add comment
    println!("\n1. Updating ticket priority from Low to High...");
    let priority_update = Ticket::builder("API Test - Ticket for Update Demo")
        .comment("Priority updated to High due to increased urgency. This is part of the API demo.")
        .priority(TicketPriority::High)
        .ticket_type(TicketType::Question)
        .status(TicketStatus::New)
        .requester_id(current_user_id)
        .build();

    match client.update_ticket(ticket_id, priority_update).await {
        Ok(updated_ticket) => {
            println!("Successfully updated ticket priority!");
            println!("   ID: {}", updated_ticket.id.unwrap_or(0));
            println!("   Subject: {}", updated_ticket.subject);
            println!("   Priority: {:?} (was Low)", updated_ticket.priority);
            println!("   Status: {:?}", updated_ticket.status);
        }
        Err(e) => {
            println!("Failed to update ticket priority: {}", e);
        }
    }

    // 2. Update ticket status to In Progress and assign to self
    println!("\n2. Updating ticket status to Open and assigning to current user...");
    let status_update = Ticket::builder("API Test - Ticket for Update Demo")
        .comment("Taking ownership of this ticket and setting status to Open. API demo continues.")
        .priority(TicketPriority::High)
        .ticket_type(TicketType::Question)
        .status(TicketStatus::Open)
        .requester_id(current_user_id)
        .assignee_id(current_user_id)
        .build();

    match client.update_ticket(ticket_id, status_update).await {
        Ok(updated_ticket) => {
            println!("Successfully updated ticket status and assignment!");
            println!("   ID: {}", updated_ticket.id.unwrap_or(0));
            println!("   Status: {:?} (was New)", updated_ticket.status);
            println!(
                "   Assignee ID: {} (was unassigned)",
                updated_ticket.assignee_id.unwrap_or(0)
            );
            println!("   Priority: {:?}", updated_ticket.priority);
        }
        Err(e) => {
            println!("Failed to update ticket status: {}", e);
        }
    }

    // 3. Add tags and change ticket type
    println!("\n3. Adding additional tags and changing ticket type to Task...");
    let tags_update = Ticket::builder("API Test - Ticket for Update Demo")
        .comment("Converting this to a Task and adding more tags for better organization.")
        .priority(TicketPriority::High)
        .ticket_type(TicketType::Task)
        .status(TicketStatus::Open)
        .requester_id(current_user_id)
        .assignee_id(current_user_id)
        .tags(vec![
            "api_test".to_string(),
            "update_demo".to_string(),
            "converted_to_task".to_string(),
            "demonstration".to_string(),
            "zendesk_api_rust".to_string(),
        ])
        .build();

    match client.update_ticket(ticket_id, tags_update).await {
        Ok(updated_ticket) => {
            println!("Successfully updated ticket type and tags!");
            println!("   ID: {}", updated_ticket.id.unwrap_or(0));
            println!("   Type: {:?} (was Question)", updated_ticket.ticket_type);
            if let Some(tags) = &updated_ticket.tags {
                println!("   Tags: {:?}", tags);
            }
        }
        Err(e) => {
            println!("Failed to update ticket tags: {}", e);
        }
    }

    // 4. Resolve the ticket
    println!("\n4. Resolving the ticket with final comment...");
    let resolve_update = Ticket::builder("API Test - Ticket for Update Demo (RESOLVED)")
        .comment("This API demonstration has been completed successfully. All update operations worked as expected. Marking as resolved.")
        .priority(TicketPriority::Normal)  // Lower priority as it's resolved
        .ticket_type(TicketType::Task)
        .status(TicketStatus::Solved)
        .requester_id(current_user_id)
        .assignee_id(current_user_id)
        .tags(vec![
            "api_test".to_string(),
            "update_demo".to_string(),
            "resolved".to_string(),
            "demonstration_complete".to_string()
        ])
        .build();

    match client.update_ticket(ticket_id, resolve_update).await {
        Ok(updated_ticket) => {
            println!("Successfully resolved the ticket!");
            println!("   ID: {}", updated_ticket.id.unwrap_or(0));
            println!("   Subject: {}", updated_ticket.subject);
            println!("   Status: {:?} (was Open)", updated_ticket.status);
            println!(
                "   Priority: {:?} (lowered from High)",
                updated_ticket.priority
            );
            if let Some(updated_at) = &updated_ticket.updated_at {
                println!("   Last Updated: {}", updated_at);
            }
        }
        Err(e) => {
            println!("Failed to resolve ticket: {}", e);
        }
    }

    // 5. Show the complete update history
    println!("\n5. Viewing final ticket state...");
    match client.get_ticket(ticket_id).await {
        Ok(final_ticket) => {
            println!("Final ticket state:");
            println!("   ID: {}", final_ticket.id.unwrap_or(0));
            println!("   Subject: {}", final_ticket.subject);
            println!("   Status: {:?}", final_ticket.status);
            println!("   Priority: {:?}", final_ticket.priority);
            println!("   Type: {:?}", final_ticket.ticket_type);
            println!("   Requester: {}", final_ticket.requester_id.unwrap_or(0));
            println!("   Assignee: {}", final_ticket.assignee_id.unwrap_or(0));
            if let Some(tags) = &final_ticket.tags {
                println!("   Tags: {:?}", tags);
            }
            if let Some(created_at) = &final_ticket.created_at {
                println!("   Created: {}", created_at);
            }
            if let Some(updated_at) = &final_ticket.updated_at {
                println!("   Last Updated: {}", updated_at);
            }
        }
        Err(e) => {
            println!("Failed to fetch final ticket state: {}", e);
        }
    }

    // 6. Demonstrate bulk updates by finding and updating multiple test tickets
    println!("\n6. Demonstrating bulk operations on test tickets...");
    match client.search_tickets("api_test").await {
        Ok(test_tickets) => {
            let open_test_tickets: Vec<_> = test_tickets
                .iter()
                .filter(|t| {
                    t.subject.contains("API Test")
                        && matches!(t.status, Some(TicketStatus::New) | Some(TicketStatus::Open))
                })
                .take(3) // Limit to 3 tickets to avoid overwhelming the API
                .collect();

            if !open_test_tickets.is_empty() {
                println!(
                    "Found {} open test tickets to bulk update",
                    open_test_tickets.len()
                );

                for ticket in open_test_tickets {
                    if let Some(tid) = ticket.id {
                        let bulk_update = Ticket::builder(&ticket.subject)
                            .comment(
                                "Bulk update: Adding standardized tag to all API test tickets.",
                            )
                            .priority(ticket.priority.clone().unwrap_or(TicketPriority::Normal))
                            .ticket_type(ticket.ticket_type.clone().unwrap_or(TicketType::Question))
                            .status(ticket.status.clone().unwrap_or(TicketStatus::Open))
                            .requester_id(ticket.requester_id.unwrap_or(current_user_id))
                            .tags(vec![
                                "api_test".to_string(),
                                "bulk_updated".to_string(),
                                "zendesk_demo".to_string(),
                            ])
                            .build();

                        match client.update_ticket(tid, bulk_update).await {
                            Ok(_) => println!("   Bulk updated ticket {}", tid),
                            Err(e) => println!("   Failed to bulk update ticket {}: {}", tid, e),
                        }
                    }
                }
            } else {
                println!("   No open test tickets found for bulk update");
            }
        }
        Err(e) => {
            println!("Failed to search for test tickets: {}", e);
        }
    }

    println!("\nTicket updating examples completed!");
    println!("\nSummary of operations performed:");
    println!("   - Created test ticket (ID: {})", ticket_id);
    println!("   - Updated priority (Low → High → Normal)");
    println!("   - Changed status (New → Open → Solved)");
    println!("   - Assigned ticket to current user");
    println!("   - Changed type (Question → Task)");
    println!("   - Added and modified tags");
    println!("   - Performed bulk updates on multiple tickets");
    println!("\nCleanup Note: Test tickets remain in your Zendesk instance for review.");

    Ok(())
}
