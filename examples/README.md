# Zendesk API Examples

This directory contains comprehensive examples demonstrating all major use cases of the Zendesk API client library.

## Available Examples

### 1. `test_api.rs` - Basic Connection Test
**Purpose**: Verify API credentials and basic connectivity
**What it demonstrates**:
- Configuration setup with API token authentication
- Basic API connection verification
- Current user information retrieval

**Usage**:
```bash
cargo run --example test_api
```

### 2. `fetch_tickets.rs` - Ticket Retrieval Operations
**Purpose**: Demonstrate various ways to fetch and search tickets
**What it demonstrates**:
- List all tickets
- Get specific ticket details by ID
- Search tickets by keyword
- Get tickets assigned to specific users
- Get tickets requested by specific users

**Usage**:
```bash
cargo run --example fetch_tickets
```

### 3. `fetch_users.rs` - User Retrieval Operations
**Purpose**: Demonstrate user account management and searching
**What it demonstrates**:
- Get current user information
- List all users in the instance
- Get detailed user information by ID
- Search users by email and other criteria
- Find users by email address
- List users within specific organizations

**Usage**:
```bash
cargo run --example fetch_users
```

### 4. `fetch_organizations.rs` - Organization Operations
**Purpose**: Demonstrate organization management and user relationships
**What it demonstrates**:
- List all organizations
- Get detailed organization information
- Search organizations by various criteria
- List users within organizations
- Organization statistics and analysis

**Usage**:
```bash
cargo run --example fetch_organizations
```

### 5. `create_tickets.rs` - Ticket Creation
**Purpose**: Demonstrate creating various types of tickets
**What it demonstrates**:
- Simple support request creation
- High priority incident tickets
- Task tickets with assignments
- Problem tickets with detailed descriptions
- Different ticket types (Question, Incident, Task, Problem)
- Priority levels (Low, Normal, High, Urgent)
- Tag management and assignment

**Usage**:
```bash
cargo run --example create_tickets
```

### 6. `update_tickets.rs` - Ticket Modification
**Purpose**: Demonstrate ticket updating and lifecycle management
**What it demonstrates**:
- Priority updates (Low → High → Normal)
- Status changes (New → Open → Solved)
- Assignment modifications
- Ticket type conversions (Question → Task)
- Tag additions and modifications
- Bulk update operations
- Complete ticket lifecycle demonstration

**Usage**:
```bash
cargo run --example update_tickets
```

### 7. `manage_users.rs` - User Account Management
**Purpose**: Demonstrate user creation, modification, and management
**What it demonstrates**:
- New user creation with full profiles
- User information updates
- User search and retrieval
- User statistics and analytics
- User deactivation approaches

**Usage**:
```bash
cargo run --example manage_users
```

### 8. `ticket_comments.rs` - Ticket Comment Retrieval
**Purpose**: Demonstrate fetching and reading ticket comments
**What it demonstrates**:
- Get all comments for a specific ticket
- Comment pagination handling
- Display comment metadata (author, timestamp, visibility)
- Comment count and statistics

**Usage**:
```bash
cargo run --example ticket_comments
```

### 9. `simple_comment_example.rs` - Basic Comment Operations
**Purpose**: Simple examples of adding comments to tickets
**What it demonstrates**:
- Add public responses (visible to customers)
- Add internal work notes (agent-only)
- Basic comment creation patterns

**Usage**:
```bash
cargo run --example simple_comment_example
```

### 10. `post_comments.rs` - Advanced Comment Operations
**Purpose**: Comprehensive comment management and ticket updates
**What it demonstrates**:
- Public customer responses
- Internal work notes
- Solving tickets with responses
- Ticket reassignment with notes
- Tag management with comments
- Comment status updates

**Usage**:
```bash
cargo run --example post_comments
```

### 11. `basic_search.rs` - Simple Search Operations
**Purpose**: Demonstrate basic search functionality across Zendesk
**What it demonstrates**:
- Text-based ticket searches
- User searches by email and name
- Organization searches
- Search result handling and display
- Search query builder basics

**Usage**:
```bash
cargo run --example basic_search
```

### 12. `advanced_search.rs` - Complex Search Operations
**Purpose**: Advanced search capabilities and filtering
**What it demonstrates**:
- Complex search queries with multiple criteria
- Search sorting and ordering
- Date range searches
- Status and priority filtering
- Search pagination
- Search result counting and export
- Advanced query builder usage

**Usage**:
```bash
cargo run --example advanced_search
```

### 13. `graph_relationships.rs` - Lookup Relationships
**Purpose**: Demonstrate lookup relationship functionality for building connected data graphs
**What it demonstrates**:
- Side-loading related resources in single API calls
- Creating lookup relationship fields
- Traversing relationships between tickets, users, and organizations
- Building data graphs from Zendesk relationships
- Advanced relationship queries

**Usage**:
```bash
cargo run --example graph_relationships
```

## Running Examples by Category

### Basic Operations
```bash
cargo run --example test_api
cargo run --example fetch_tickets
cargo run --example fetch_users
cargo run --example fetch_organizations
```

### Ticket Management
```bash
cargo run --example create_tickets
cargo run --example update_tickets
cargo run --example ticket_comments
cargo run --example simple_comment_example
cargo run --example post_comments
```

### Search and Discovery
```bash
cargo run --example basic_search
cargo run --example advanced_search
```

### Advanced Features
```bash
cargo run --example manage_users
cargo run --example graph_relationships
```

## Configuration

All examples use environment variables for configuration. Set up your `.env` file:

```env
ZENDESK_SUBDOMAIN=your-subdomain
ZENDESK_EMAIL=your-email@domain.com
ZENDESK_API_TOKEN=your-api-token
```

Examples load configuration like this:

```rust
let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
let api_token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

let config = ZendeskConfig::new(subdomain, AuthMethod::api_token(email, api_token));
let client = ZendeskClient::new(config)?;
```

## Prerequisites

- Valid Zendesk instance with API access enabled
- API token generated in Zendesk Admin Center
- Environment variables configured
- Some examples require existing tickets/users (will be noted in example output)

## Cleanup

Most examples create test data that remains in your Zendesk instance:
- Test tickets can be deleted manually or via API
- Test users may require admin privileges to delete
- Search for the "api_test" tag to find all test data
