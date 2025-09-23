use std::env;
use zendesk_api_rust::{
    ZendeskClient, ZendeskConfig,
    auth::AuthMethod,
    models::{
        organization::Organization,
        relationship::{CreateLookupRelationshipField, ZendeskObjectType},
        user::User,
    },
    query::QueryParams,
    query::SortOrder,
};

/// Example demonstrating graph relationship capabilities of the Zendesk API client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    // Initialize the client
    let auth = AuthMethod::api_token(&email, &token);
    let config = ZendeskConfig::new(&subdomain, auth);
    let client = ZendeskClient::new(config)?;

    println!("Zendesk Graph Relationship Examples");
    println!("=====================================\n");

    // Example 1: Side-loading - Fetch tickets with related data in a single call
    example_sideloading(&client).await?;

    // Example 2: Create lookup relationship fields
    example_create_lookup_relationships(&client).await?;

    // Example 3: Traverse lookup relationships to build a graph
    example_graph_traversal(&client).await?;

    // Example 4: Advanced graph analysis
    example_advanced_graph_analysis(&client).await?;

    Ok(())
}

/// Example 1: Demonstrates side-loading to efficiently fetch related resources
async fn example_sideloading(client: &ZendeskClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Side-loading Related Resources");
    println!("---------------------------------------------");

    // Without side-loading: Multiple API calls required
    println!("Inefficient approach (multiple API calls):");
    let tickets = client.list_tickets().await?;
    println!("   Fetched {} tickets", tickets.len());

    // This would require N additional API calls for users + M calls for organizations
    // For 1000 tickets, this could be 2000+ API calls!

    // With side-loading: Single API call gets everything
    println!("\nEfficient approach (single API call with side-loading):");
    let tickets_with_sideloading = client
        .list_tickets_with_sideloading(&["users", "organizations", "groups"])
        .await?;

    println!(
        "   Fetched {} tickets",
        tickets_with_sideloading.primary.tickets.len()
    );

    if let Some(users) = tickets_with_sideloading.users() {
        println!("   Side-loaded {} users", users.len());
    }

    if let Some(orgs) = tickets_with_sideloading.organizations() {
        println!("   Side-loaded {} organizations", orgs.len());
    }

    // Access relationships efficiently
    for ticket in tickets_with_sideloading.primary.tickets.iter().take(3) {
        if let Some(ticket_id) = ticket.id {
            println!("\n   Ticket #{}: {}", ticket_id, ticket.subject);

            if let Some(requester_id) = ticket.requester_id {
                // Find the requester in side-loaded users
                if let Some(requester) =
                    tickets_with_sideloading.find_sideloaded::<User>("users", requester_id)
                {
                    println!("     Requester: {} ({})", requester.name, requester.email);
                }
            }

            if let Some(org_id) = ticket.organization_id {
                // Find the organization in side-loaded data
                if let Some(org) = tickets_with_sideloading
                    .find_sideloaded::<Organization>("organizations", org_id)
                {
                    println!("     Organization: {}", org.name);
                }
            }
        }
    }

    println!("\nSide-loading reduces API calls from 2000+ to just 1!\n");
    Ok(())
}

/// Example 2: Create lookup relationship fields for custom relationships
async fn example_create_lookup_relationships(
    client: &ZendeskClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Creating Lookup Relationship Fields");
    println!("--------------------------------------------------");

    // Create a lookup field that links tickets to account managers (users)
    println!("Creating 'Account Manager' lookup field for tickets...");
    let account_manager_field =
        CreateLookupRelationshipField::builder("Account Manager", ZendeskObjectType::User)
            .description("The account manager responsible for this ticket")
            .filter_users_by_role("agent") // Only show agents in the dropdown
            .required(false)
            .build();

    match client
        .create_ticket_lookup_field(account_manager_field)
        .await
    {
        Ok(field) => {
            println!("Created Account Manager field (ID: {})", field.id);
        }
        Err(e) => {
            println!("Field might already exist: {}", e);
        }
    }

    // Create a lookup field that links tickets to related products (custom objects)
    println!("\nCreating 'Related Product' lookup field for tickets...");
    let product_field =
        CreateLookupRelationshipField::custom_object_lookup("Related Product", "product");

    match client.create_ticket_lookup_field(product_field).await {
        Ok(field) => {
            println!("Created Related Product field (ID: {})", field.id);
        }
        Err(e) => {
            println!("Field might already exist: {}", e);
        }
    }

    // Create a lookup field that links organizations to account managers
    println!("\nCreating 'Account Manager' lookup field for organizations...");
    let org_account_manager_field =
        CreateLookupRelationshipField::builder("Account Manager", ZendeskObjectType::User)
            .description("The primary account manager for this organization")
            .filter_users_by_role("agent")
            .required(true)
            .build();

    match client
        .create_organization_lookup_field(org_account_manager_field)
        .await
    {
        Ok(field) => {
            println!(
                "Created Organization Account Manager field (ID: {})",
                field.id
            );
        }
        Err(e) => {
            println!("Field might already exist: {}", e);
        }
    }

    println!("\nLookup relationship fields enable custom graph connections!\n");
    Ok(())
}

/// Example 3: Traverse lookup relationships to build graph connections
async fn example_graph_traversal(client: &ZendeskClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Graph Traversal via Lookup Relationships");
    println!("--------------------------------------------------------");

    // First, get all available lookup fields
    let ticket_fields = client.list_ticket_lookup_fields().await?;
    println!("Found {} lookup fields for tickets", ticket_fields.len());

    for field in &ticket_fields {
        println!(
            "  - {} (ID: {}) -> {}",
            field.title,
            field.id,
            field.relationship_target_type.as_api_string()
        );
    }

    if let Some(account_manager_field) = ticket_fields
        .iter()
        .find(|f| f.title.contains("Account Manager"))
    {
        println!("\nFinding tickets related to specific account managers...");

        // Let's say we want to find all tickets managed by user ID 123
        let user_id = 123u64; // Replace with actual user ID
        let field_id = account_manager_field.id;

        match client.get_tickets_related_to_user(user_id, field_id).await {
            Ok(related_tickets) => {
                println!(
                    "Found {} tickets managed by user {}",
                    related_tickets.results.len(),
                    user_id
                );

                // Build graph connections
                for ticket in related_tickets.results.iter().take(5) {
                    if let Some(ticket_id) = ticket.id {
                        println!("  Ticket #{}: {}", ticket_id, ticket.subject);

                        // This creates a graph edge: User(123) --manages--> Ticket(ticket_id)
                        println!(
                            "     Graph Edge: User({}) --manages--> Ticket({})",
                            user_id, ticket_id
                        );
                    }
                }
            }
            Err(e) => {
                println!("Could not fetch related tickets: {}", e);
            }
        }
    }

    // Demonstrate reverse lookup: Find users related to a specific ticket
    println!("\nReverse lookup: Finding users related to tickets...");

    // Get a sample ticket first
    if let Ok(tickets) = client.list_tickets().await {
        if let Some(sample_ticket) = tickets.first() {
            if let Some(ticket_id) = sample_ticket.id {
                for field in &ticket_fields {
                    if field.relationship_target_type == ZendeskObjectType::User {
                        match client
                            .get_users_related_to_ticket(ticket_id, field.id)
                            .await
                        {
                            Ok(related_users) => {
                                if !related_users.results.is_empty() {
                                    println!(
                                        "  Ticket {} has {} related users via '{}'",
                                        ticket_id,
                                        related_users.results.len(),
                                        field.title
                                    );
                                }
                            }
                            Err(_) => {
                                // Field might not have any relationships yet
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\nGraph traversal reveals hidden connections between entities!\n");
    Ok(())
}

/// Example 4: Advanced graph analysis using multiple relationship types
async fn example_advanced_graph_analysis(
    client: &ZendeskClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Advanced Graph Analysis");
    println!("--------------------------------------");

    // Analyze ticket patterns with side-loaded relationship data
    let params = QueryParams::new()
        .with_sideloading(&["users", "organizations"])
        .with_per_page(100);

    let tickets_response = client.list_tickets_with_params(&params).await?;
    let tickets = tickets_response.tickets;

    println!("Analyzing {} tickets for graph patterns...", tickets.len());

    // Build a simple graph analysis
    let mut user_ticket_count = std::collections::HashMap::new();
    let mut org_ticket_count = std::collections::HashMap::new();
    let mut status_distribution = std::collections::HashMap::new();
    let mut priority_distribution = std::collections::HashMap::new();

    for ticket in &tickets {
        // Count tickets per user (requesters)
        if let Some(requester_id) = ticket.requester_id {
            *user_ticket_count.entry(requester_id).or_insert(0) += 1;
        }

        // Count tickets per organization
        if let Some(org_id) = ticket.organization_id {
            *org_ticket_count.entry(org_id).or_insert(0) += 1;
        }

        // Analyze status distribution
        if let Some(ref status) = ticket.status {
            let status_str = format!("{:?}", status);
            *status_distribution.entry(status_str).or_insert(0) += 1;
        }

        // Analyze priority distribution
        if let Some(ref priority) = ticket.priority {
            let priority_str = format!("{:?}", priority);
            *priority_distribution.entry(priority_str).or_insert(0) += 1;
        }
    }

    // Find power users (users with most tickets)
    let mut user_counts: Vec<_> = user_ticket_count.into_iter().collect();
    user_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop 5 Users by Ticket Count:");
    for (user_id, count) in user_counts.iter().take(5) {
        println!("  User {}: {} tickets", user_id, count);
        // In a real scenario, you'd fetch user details to show names
    }

    // Find organizations with most tickets
    let mut org_counts: Vec<_> = org_ticket_count.into_iter().collect();
    org_counts.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop 5 Organizations by Ticket Count:");
    for (org_id, count) in org_counts.iter().take(5) {
        println!("  Organization {}: {} tickets", org_id, count);
    }

    // Status distribution
    println!("\nTicket Status Distribution:");
    for (status, count) in &status_distribution {
        println!("  {}: {} tickets", status, count);
    }

    // Priority distribution
    println!("\nTicket Priority Distribution:");
    for (priority, count) in &priority_distribution {
        println!("  {}: {} tickets", priority, count);
    }

    // Demonstrate complex relationship query
    println!("\nComplex Relationship Analysis:");
    println!("Finding tickets with custom field relationships...");

    // Get tickets with their custom field relationships
    let sample_ticket_ids: Vec<u64> = tickets.iter().filter_map(|t| t.id).take(5).collect();

    if !sample_ticket_ids.is_empty() {
        let lookup_fields = client.list_ticket_lookup_fields().await?;
        let field_specs: Vec<(u64, ZendeskObjectType)> = lookup_fields
            .iter()
            .map(|f| (f.id, f.relationship_target_type.clone()))
            .collect();

        if !field_specs.is_empty() {
            match client
                .get_tickets_with_lookup_relationships(&sample_ticket_ids, &field_specs)
                .await
            {
                Ok(tickets_with_relationships) => {
                    println!(
                        "Analyzed {} tickets with their custom relationships",
                        tickets_with_relationships.len()
                    );

                    for (ticket, relationships) in &tickets_with_relationships {
                        if let Some(ticket_id) = ticket.id {
                            println!(
                                "  📋 Ticket {}: {} custom relationships",
                                ticket_id,
                                relationships.len()
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("Complex relationship query failed: {}", e);
                }
            }
        }
    }

    println!("\nGraph analysis reveals:");
    println!("  - Customer engagement patterns");
    println!("  - Organizational support needs");
    println!("  - Workload distribution");
    println!("  - Custom relationship insights");

    Ok(())
}

/// Helper function to demonstrate query parameters
#[allow(dead_code)]
async fn example_query_parameters(
    client: &ZendeskClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Query Parameters Examples");
    println!("----------------------------");

    // Basic pagination
    let params = QueryParams::new().with_page(1).with_per_page(25);

    let _tickets = client.list_tickets_with_params(&params).await?;

    // Side-loading with pagination and sorting
    let params = QueryParams::new()
        .with_include(vec!["users".to_string()])
        .with_page(1)
        .with_per_page(25)
        .with_sort("updated_at".to_string(), SortOrder::Desc);

    let _tickets = client.list_tickets_with_params(&params).await?;

    println!("Query parameters allow fine-tuned data fetching");
    Ok(())
}
