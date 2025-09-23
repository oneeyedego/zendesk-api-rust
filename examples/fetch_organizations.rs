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

    println!("Fetching Organizations Examples\n");

    // 1. List all organizations
    println!("1. Listing all organizations...");
    match client.list_organizations().await {
        Ok(organizations) => {
            println!("Found {} organizations", organizations.len());

            if organizations.is_empty() {
                println!("   No organizations found in this Zendesk instance.");
            } else {
                for (i, org) in organizations.iter().take(5).enumerate() {
                    println!("   {}. {} (ID: {})", i + 1, org.name, org.id.unwrap_or(0));
                    if let Some(domain_names) = &org.domain_names {
                        if !domain_names.is_empty() {
                            println!("      Domains: {:?}", domain_names);
                        }
                    }
                }
                if organizations.len() > 5 {
                    println!("   ... and {} more organizations", organizations.len() - 5);
                }

                // Get detailed info for first organization
                if let Some(first_org) = organizations.first() {
                    if let Some(org_id) = first_org.id {
                        println!(
                            "\n2. Fetching detailed info for organization ID {}...",
                            org_id
                        );
                        match client.get_organization(org_id).await {
                            Ok(org_detail) => {
                                println!("Organization Details:");
                                println!("   Name: {}", org_detail.name);
                                if let Some(details) = &org_detail.details {
                                    println!("   Details: {}", details);
                                }
                                if let Some(notes) = &org_detail.notes {
                                    let short_notes = if notes.len() > 100 {
                                        format!("{}...", &notes[..100])
                                    } else {
                                        notes.clone()
                                    };
                                    println!("   Notes: {}", short_notes);
                                }
                                if let Some(domain_names) = &org_detail.domain_names {
                                    println!("   Domain Names: {:?}", domain_names);
                                }
                                if let Some(tags) = &org_detail.tags {
                                    println!("   Tags: {:?}", tags);
                                }
                                if let Some(created_at) = &org_detail.created_at {
                                    println!("   Created: {}", created_at);
                                }
                                if let Some(updated_at) = &org_detail.updated_at {
                                    println!("   Updated: {}", updated_at);
                                }
                            }
                            Err(e) => println!("Failed to fetch organization details: {}", e),
                        }

                        // 3. Get users in this organization
                        println!("\n3. Fetching users in organization {}...", org_id);
                        match client.list_users_in_organization(org_id).await {
                            Ok(org_users) => {
                                println!("Found {} users in this organization", org_users.len());
                                for (i, user) in org_users.iter().take(5).enumerate() {
                                    println!(
                                        "   {}. {} <{}> (Role: {:?})",
                                        i + 1,
                                        user.name,
                                        &user.email,
                                        user.role
                                    );
                                }
                                if org_users.len() > 5 {
                                    println!("   ... and {} more users", org_users.len() - 5);
                                }
                            }
                            Err(e) => println!("Failed to fetch organization users: {}", e),
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to list organizations: {}", e);
        }
    }

    // 4. Search organizations by name
    println!("\n4. Searching organizations with keyword 'test'...");
    match client.search_organizations("test").await {
        Ok(search_results) => {
            println!(
                "Found {} organizations matching 'test'",
                search_results.len()
            );
            for org in search_results.iter().take(3) {
                println!("   - {} (ID: {})", org.name, org.id.unwrap_or(0));
            }
        }
        Err(e) => {
            println!("Organization search failed: {}", e);
        }
    }

    // 5. Alternative search by domain
    println!("\n5. Searching organizations by domain...");
    match client.search_organizations("domain:example.com").await {
        Ok(domain_results) => {
            if domain_results.is_empty() {
                println!("No organizations found with domain 'example.com'");
            } else {
                println!(
                    "Found {} organizations with domain 'example.com'",
                    domain_results.len()
                );
                for org in domain_results.iter() {
                    println!("   - {} (ID: {})", org.name, org.id.unwrap_or(0));
                }
            }
        }
        Err(e) => {
            println!("Domain search failed: {}", e);
        }
    }

    // 6. Show organization statistics
    println!("\n6. Organization Statistics Summary");
    match client.list_organizations().await {
        Ok(orgs) => {
            println!("Total Organizations: {}", orgs.len());

            // Count organizations with domains
            let orgs_with_domains = orgs
                .iter()
                .filter(|org| org.domain_names.as_ref().map_or(false, |d| !d.is_empty()))
                .count();
            println!("   Organizations with domains: {}", orgs_with_domains);

            // Count organizations with tags
            let orgs_with_tags = orgs
                .iter()
                .filter(|org| org.tags.as_ref().map_or(false, |t| !t.is_empty()))
                .count();
            println!("   Organizations with tags: {}", orgs_with_tags);

            // Show unique domains
            let mut all_domains = Vec::new();
            for org in &orgs {
                if let Some(domains) = &org.domain_names {
                    all_domains.extend(domains.iter().cloned());
                }
            }
            all_domains.sort();
            all_domains.dedup();

            if !all_domains.is_empty() {
                println!("   Unique domains: {:?}", all_domains);
            }
        }
        Err(e) => println!("Failed to get organizations for statistics: {}", e),
    }

    println!("\nOrganization fetching examples completed!");

    Ok(())
}
