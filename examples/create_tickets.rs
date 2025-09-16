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

    println!("🎫 Creating Tickets Examples\n");

    // Get current user ID for ticket creation
    let current_user_id = match client.get::<serde_json::Value>("users/me.json").await {
        Ok(user_data) => user_data
            .get("user")
            .and_then(|u| u.get("id"))
            .and_then(|id| id.as_u64())
            .ok_or("Could not get current user ID")?,
        Err(e) => {
            println!("❌ Failed to get current user: {}", e);
            return Err(e.into());
        }
    };

    println!("Current user ID: {}\n", current_user_id);

    // 1. Create a simple ticket
    println!("1. Creating a simple support ticket...");
    let simple_ticket = Ticket::builder("API Test - Simple Support Request")
        .comment("This is a test ticket created via the Zendesk API. Please ignore.")
        .priority(TicketPriority::Normal)
        .ticket_type(TicketType::Question)
        .requester_id(current_user_id)
        .tags(vec!["api_test".to_string(), "automated".to_string()])
        .build();

    match client.create_ticket(simple_ticket).await {
        Ok(created_ticket) => {
            println!("✓ Successfully created ticket!");
            println!("   ID: {}", created_ticket.id.unwrap_or(0));
            println!("   Subject: {}", created_ticket.subject);
            println!("   Status: {:?}", created_ticket.status);
            println!("   Priority: {:?}", created_ticket.priority);
            if let Some(url) = &created_ticket.url {
                println!("   URL: {}", url);
            }
        }
        Err(e) => {
            println!("❌ Failed to create simple ticket: {}", e);
        }
    }

    // 2. Create a high priority incident ticket
    println!("\n2. Creating a high priority incident ticket...");
    let incident_ticket = Ticket::builder("API Test - Critical System Outage")
        .comment("URGENT: This is a test incident ticket. The system appears to be experiencing issues. This is just a test - please ignore.")
        .priority(TicketPriority::High)
        .ticket_type(TicketType::Incident)
        .status(TicketStatus::Open)
        .requester_id(current_user_id)
        .tags(vec!["incident".to_string(), "api_test".to_string(), "urgent".to_string()])
        .build();

    match client.create_ticket(incident_ticket).await {
        Ok(created_ticket) => {
            println!("✓ Successfully created incident ticket!");
            println!("   ID: {}", created_ticket.id.unwrap_or(0));
            println!("   Subject: {}", created_ticket.subject);
            println!("   Status: {:?}", created_ticket.status);
            println!("   Priority: {:?}", created_ticket.priority);
            println!("   Type: {:?}", created_ticket.ticket_type);
            if let Some(tags) = &created_ticket.tags {
                println!("   Tags: {:?}", tags);
            }
        }
        Err(e) => {
            println!("❌ Failed to create incident ticket: {}", e);
        }
    }

    // 3. Create a task ticket with assignment
    println!("\n3. Creating a task ticket assigned to current user...");
    let task_ticket = Ticket::builder("API Test - Weekly System Maintenance")
        .comment("This is a scheduled maintenance task created via API. Please complete the weekly system checks. This is just a test.")
        .priority(TicketPriority::Low)
        .ticket_type(TicketType::Task)
        .status(TicketStatus::New)
        .requester_id(current_user_id)
        .assignee_id(current_user_id)  // Assign to self
        .tags(vec!["maintenance".to_string(), "api_test".to_string(), "weekly".to_string()])
        .build();

    match client.create_ticket(task_ticket).await {
        Ok(created_ticket) => {
            println!("✓ Successfully created task ticket!");
            println!("   ID: {}", created_ticket.id.unwrap_or(0));
            println!("   Subject: {}", created_ticket.subject);
            println!("   Status: {:?}", created_ticket.status);
            println!("   Priority: {:?}", created_ticket.priority);
            println!("   Type: {:?}", created_ticket.ticket_type);
            println!(
                "   Requester ID: {}",
                created_ticket.requester_id.unwrap_or(0)
            );
            println!(
                "   Assignee ID: {}",
                created_ticket.assignee_id.unwrap_or(0)
            );
        }
        Err(e) => {
            println!("❌ Failed to create task ticket: {}", e);
        }
    }

    // 4. Create a problem ticket with detailed description
    println!("\n4. Creating a problem ticket with detailed description...");
    let problem_description = r#"
**Problem Description:**
Users are reporting intermittent login failures when accessing the application.

**Steps to Reproduce:**
1. Navigate to login page
2. Enter valid credentials
3. Click login button
4. Observe timeout error

**Expected Behavior:**
User should be logged in successfully

**Actual Behavior:**
Login times out after 30 seconds

**Environment:**
- Browser: Chrome 120+
- OS: Various
- Time: Peak hours (9-11 AM)

**Additional Notes:**
This appears to be affecting approximately 15% of login attempts during peak hours.

*Note: This is a test ticket created via API - please ignore.*
    "#
    .trim();

    let problem_ticket = Ticket::builder("API Test - Login Timeout Issues During Peak Hours")
        .comment(problem_description)
        .priority(TicketPriority::High)
        .ticket_type(TicketType::Problem)
        .status(TicketStatus::New)
        .requester_id(current_user_id)
        .tags(vec![
            "login_issue".to_string(),
            "api_test".to_string(),
            "performance".to_string(),
            "peak_hours".to_string(),
        ])
        .build();

    match client.create_ticket(problem_ticket).await {
        Ok(created_ticket) => {
            println!("✓ Successfully created problem ticket!");
            println!("   ID: {}", created_ticket.id.unwrap_or(0));
            println!("   Subject: {}", created_ticket.subject);
            println!("   Status: {:?}", created_ticket.status);
            println!("   Priority: {:?}", created_ticket.priority);
            println!("   Type: {:?}", created_ticket.ticket_type);
            if let Some(desc) = &created_ticket.description {
                let short_desc = if desc.len() > 100 {
                    format!("{}...", &desc[..100])
                } else {
                    desc.clone()
                };
                println!("   Description: {}", short_desc);
            }
        }
        Err(e) => {
            println!("❌ Failed to create problem ticket: {}", e);
        }
    }

    // 5. Show summary of created tickets
    println!("\n5. Showing recently created test tickets...");
    match client.search_tickets("api_test").await {
        Ok(test_tickets) => {
            let recent_tickets: Vec<_> = test_tickets
                .iter()
                .filter(|ticket| ticket.subject.contains("API Test"))
                .take(10)
                .collect();

            println!("✓ Found {} recent API test tickets:", recent_tickets.len());
            for (i, ticket) in recent_tickets.iter().enumerate() {
                println!(
                    "   {}. {} (ID: {}, Status: {:?}, Priority: {:?})",
                    i + 1,
                    ticket.subject,
                    ticket.id.unwrap_or(0),
                    ticket.status,
                    ticket.priority
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to search for test tickets: {}", e);
        }
    }

    println!("\n✅ Ticket creation examples completed!");
    println!("\n📋 Summary:");
    println!("   - Created simple support request");
    println!("   - Created high priority incident");
    println!("   - Created assigned task");
    println!("   - Created detailed problem report");
    println!(
        "\n🧹 Cleanup Note: You may want to delete these test tickets from your Zendesk instance."
    );

    Ok(())
}
