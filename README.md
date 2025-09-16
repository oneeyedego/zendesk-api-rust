# Zendesk API Rust Client

A Rust client library for the Zendesk ticketing system API.

## Features

- Asynchronous API calls using `tokio` and `reqwest`
- Support for API token authentication
- Endpoints for tickets, users, and organizations
- Type-safe models with `serde` serialization
- Comprehensive error handling

## Quick Start

```rust
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};
use zendesk_api_rust::auth::AuthMethod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create authentication
    let auth = AuthMethod::api_token("your-email@example.com", "your-api-token");

    // Create configuration
    let config = ZendeskConfig::new("your-subdomain", auth);

    // Create client
    let client = ZendeskClient::new(config)?;

    // Make API calls
    let user_data = client.get::<serde_json::Value>("users/me.json").await?;
    println!("{:?}", user_data);

    Ok(())
}
```

## Examples

See the `examples/` directory for usage examples:

- `test_api.rs` - Basic API connection test
- `fetch_tickets.rs` - Retrieve tickets
- `fetch_users.rs` - User management
- `create_tickets.rs` - Create new tickets
- `manage_users.rs` - User operations
- `fetch_organizations.rs` - Organization data

## License

MIT
