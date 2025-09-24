# Zendesk API Rust Client

A Rust client library for the Zendesk ticketing system API.

## Features

- Asynchronous API calls using `tokio` and `reqwest`
- Models and endpoints for tickets, users, organizations, custom objects, lookup relationships, and search
- Support for CRUD operations, sideloading, pagination, filtering, and advanced queries
- Comprehensive ticket management including comments, tags, and workflow helpers
- Advanced search capabilities with query builders and type-specific helpers
- Full lookup relationship support for connecting Zendesk objects
- Built-in helpers for common operations and bulk relationship traversal

## API Coverage

This library implements the following Zendesk API endpoints:

### Tickets
**Implemented:**
- `GET /api/v2/tickets` - List tickets (with sideloading and pagination support)
- `GET /api/v2/tickets/{ticket_id}` - Show ticket (with sideloading support)
- `POST /api/v2/tickets` - Create ticket
- `PUT /api/v2/tickets/{ticket_id}` - Update ticket
- `DELETE /api/v2/tickets/{ticket_id}` - Delete ticket
- `GET /api/v2/users/{user_id}/tickets/assigned` - List tickets assigned to user (with sideloading)
- `GET /api/v2/users/{user_id}/tickets/requested` - List tickets requested by user (with sideloading)
- `GET /api/v2/tickets/{ticket_id}/comments` - List ticket comments (with sideloading and pagination)
- `POST /api/v2/tickets/{ticket_id}/comments` - Add ticket comment
- `PUT /api/v2/tickets/{ticket_id}/comments/{comment_id}/make_private` - Make comment private
- Tag management: Add, remove, and replace ticket tags
- Advanced comment operations: Public responses, work notes, comments with updates
- Ticket workflow helpers: Solve with response, reassign with note

**Not implemented:**
- `GET /api/v2/tickets/recent` - List recent tickets
- `GET /api/v2/tickets/count` - Count tickets
- `GET /api/v2/tickets/show_many` - Show multiple tickets
- `POST /api/v2/tickets/create_many` - Create multiple tickets
- `PUT /api/v2/tickets/update_many` - Update multiple tickets
- `PUT /api/v2/tickets/{ticket_id}/mark_as_spam` - Mark ticket as spam
- `POST /api/v2/tickets/{ticket_id}/merge` - Merge tickets

### Users
**Implemented:**
- `GET /api/v2/users` - List users
- `GET /api/v2/users/{user_id}` - Show user
- `GET /api/v2/users/search?query=email:{email}` - Get user by email
- `POST /api/v2/users` - Create user
- `PUT /api/v2/users/{user_id}` - Update user
- `DELETE /api/v2/users/{user_id}` - Delete user
- `GET /api/v2/users/search` - Search users
- `GET /api/v2/organizations/{organization_id}/users` - List users in organization

**Not implemented:**
- `GET /api/v2/users/me` - Show current user
- `GET /api/v2/users/count` - Count users
- `GET /api/v2/users/show_many` - Show multiple users
- `GET /api/v2/users/autocomplete` - Autocomplete users
- `POST /api/v2/users/create_many` - Create multiple users
- `POST /api/v2/users/create_or_update` - Create or update user
- `GET /api/v2/users/{user_id}/related` - Show related user information

### Organizations
**Implemented:**
- `GET /api/v2/organizations` - List organizations
- `GET /api/v2/organizations/{organization_id}` - Show organization
- `POST /api/v2/organizations` - Create organization
- `PUT /api/v2/organizations/{organization_id}` - Update organization
- `DELETE /api/v2/organizations/{organization_id}` - Delete organization
- `GET /api/v2/organizations/search?name={name}` - Search organizations by name
- `GET /api/v2/organizations/search?external_id={id}` - Search organizations by external ID
- `GET /api/v2/organizations/search` - Search organizations (general)

**Not implemented:**
- `GET /api/v2/organizations/count` - Count organizations
- `GET /api/v2/organizations/show_many` - Show multiple organizations
- `GET /api/v2/organizations/autocomplete` - Autocomplete organizations
- `POST /api/v2/organizations/create_many` - Create multiple organizations
- `PUT /api/v2/organizations/update_many` - Update multiple organizations
- `POST /api/v2/organizations/{organization_id}/merge` - Merge organizations
- `GET /api/v2/organizations/{organization_id}/related` - Show related organization information

### Custom Objects
**Implemented:**
- `GET /api/v2/custom_objects` - List custom objects
- `GET /api/v2/custom_objects/{custom_object_key}` - Show custom object
- `POST /api/v2/custom_objects` - Create custom object
- `PUT /api/v2/custom_objects/{custom_object_key}` - Update custom object
- `DELETE /api/v2/custom_objects/{custom_object_key}` - Delete custom object
- `GET /api/v2/custom_objects/limits/object_limit` - Check custom objects limit
- `GET /api/v2/custom_objects/{key}/fields` - List custom object fields
- `POST /api/v2/custom_objects/{key}/fields` - Create custom object field
- `GET /api/v2/custom_objects/{key}/fields/{field_id}` - Show custom object field
- `PATCH /api/v2/custom_objects/{key}/fields/{field_id}` - Update custom object field
- `DELETE /api/v2/custom_objects/{key}/fields/{field_id}` - Delete custom object field
- `PUT /api/v2/custom_objects/{key}/fields/reorder` - Reorder custom object fields
- `GET /api/v2/custom_objects/{key}/limits/field_limit` - Check field limits
- `GET /api/v2/custom_objects/{key}/records` - List custom object records (with filtering/sorting)
- `GET /api/v2/custom_objects/{key}/records/{record_id}` - Show custom object record
- `POST /api/v2/custom_objects/{key}/records` - Create custom object record
- `PATCH /api/v2/custom_objects/{key}/records/{record_id}` - Update custom object record
- `PATCH /api/v2/custom_objects/{key}/records` - Upsert custom object record (by external ID/name)
- `DELETE /api/v2/custom_objects/{key}/records/{record_id}` - Delete custom object record
- `GET /api/v2/custom_objects/{key}/records/count` - Count custom object records
- `GET /api/v2/custom_objects/{key}/records/search` - Search custom object records (GET)
- `POST /api/v2/custom_objects/{key}/records/search` - Search custom object records (POST with filters)
- `POST /api/v2/custom_objects/{key}/jobs` - Create bulk job for records
- `GET /api/v2/custom_objects/{key}/jobs/{job_id}` - Get bulk job status
- `GET /api/v2/incremental/custom_objects/{key}/cursor` - Incremental export of records

**Field Types Supported:**
- Text, textarea, checkbox, date, integer, decimal, regexp, dropdown, lookup, multiselect
- Full validation and relationship support

**Not implemented:**
- Custom object webhooks and triggers

### Search
**Implemented:**
- `GET /api/v2/search` - Search (with sorting and pagination support)
- `GET /api/v2/search/count` - Count search results
- `GET /api/v2/search/export` - Export search results (with cursor pagination)
- Advanced search methods for tickets, users, organizations, and groups
- Search query builder for complex queries
- Type-specific search helpers (tickets, users, organizations advanced search)

### Lookup Relationships
**Implemented:**
- `GET /api/v2/{object_type}/{id}/relationship_fields/{field_id}/{source_type}` - Get sources by target (with sideloading and pagination)
- `POST /api/v2/ticket_fields` - Create ticket lookup field
- `POST /api/v2/user_fields` - Create user lookup field
- `POST /api/v2/organization_fields` - Create organization lookup field
- `GET /api/v2/ticket_fields` - List all ticket lookup fields
- `GET /api/v2/user_fields` - List all user lookup fields
- `GET /api/v2/organization_fields` - List all organization lookup fields
- `GET /api/v2/ticket_fields/{field_id}` - Get ticket lookup field
- `GET /api/v2/user_fields/{field_id}` - Get user lookup field
- `GET /api/v2/organization_fields/{field_id}` - Get organization lookup field
- `DELETE /api/v2/ticket_fields/{field_id}` - Delete ticket lookup field
- `DELETE /api/v2/user_fields/{field_id}` - Delete user lookup field
- `DELETE /api/v2/organization_fields/{field_id}` - Delete organization lookup field
- Helper methods for common relationship queries (tickets by user, users by ticket, etc.)
- Advanced relationship traversal with bulk operations

## Getting Started

### Prerequisites

- Rust 1.70+ installed
- A Zendesk account with API access

### Environment Variables

Set up the following environment variables in your `.env` file or system environment:

```env
ZENDESK_SUBDOMAIN=your-subdomain
ZENDESK_EMAIL=your-email@example.com
ZENDESK_API_TOKEN=your-api-token
```

To generate an API token:
1. Go to your Zendesk Admin Center
2. Navigate to Apps and integrations > APIs > Zendesk API
3. Enable token access and add a new API token

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zendesk-api-rust = { git = "https://github.com/your-repo/zendesk-api-rust" }
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};
use zendesk_api_rust::auth::AuthMethod;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN")?;
    let email = env::var("ZENDESK_EMAIL")?;
    let token = env::var("ZENDESK_API_TOKEN")?;

    // Create authentication
    let auth = AuthMethod::api_token(&email, &token);

    // Create configuration
    let config = ZendeskConfig::new(&subdomain, auth);

    // Create client
    let client = ZendeskClient::new(config)?;

    // Test connection by getting current user
    let user = client.get_user_by_email(&email).await?;
    println!("Connected as: {} (ID: {})", user.name, user.id);

    Ok(())
}
```

### Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd zendesk-api-rust

# Set up environment variables
cp .env.example .env
# Edit .env with your credentials

# Build the project
cargo build

# Run tests (requires valid credentials)
cargo test

# Run examples
cargo run --example basic_search
```

## Examples

See the `examples/` directory for usage examples.

## License

MIT
