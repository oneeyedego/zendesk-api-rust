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

**Key Features**:
- Shows ticket details including status, priority, tags
- Demonstrates different search and filtering methods
- Handles pagination and large result sets

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

**Key Features**:
- User role identification (Admin, Agent, End User)
- User profile details (timezone, locale, phone, etc.)
- Organization membership information

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

**Key Features**:
- Organization domain management
- User-organization relationships
- Tags and metadata handling

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

**Key Features**:
- Ticket builder pattern for easy construction
- Multiple ticket types and priorities
- Automatic requester and assignee handling
- Tag and metadata assignment

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

**Key Features**:
- Step-by-step ticket progression
- Comment addition with updates
- Assignment and ownership changes
- Bulk operations on multiple tickets

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

**Key Features**:
- Complete user profile management
- Role assignments and changes
- Contact information handling
- User lifecycle management

## Running All Examples

To test all examples in sequence:

```bash
# Test basic connection
cargo run --example test_api

# Test all fetch operations
cargo run --example fetch_tickets
cargo run --example fetch_users
cargo run --example fetch_organizations

# Test creation and modification
cargo run --example create_tickets
cargo run --example update_tickets
cargo run --example manage_users
```

## Configuration

All examples use the same configuration pattern:

```rust
let subdomain = "your-subdomain";  // e.g., "company" for company.zendesk.com
let email = "your-email@domain.com";
let token = "your-api-token";

let auth = AuthMethod::api_token(email, token);
let config = ZendeskConfig::new(subdomain, auth);
let client = ZendeskClient::new(config)?;
```

## Test Data

The examples create test data with the following patterns:
- **Tickets**: Prefixed with "API Test -" for easy identification
- **Users**: Use "demo" or "test" in names/emails
- **Tags**: Include "api_test" tag for filtering

## Cleanup

Most examples create test data that remains in your Zendesk instance:
- Test tickets can be deleted manually or via API
- Test users may require admin privileges to delete
- Search for "api_test" tag to find all test data

## Error Handling

Examples demonstrate:
- Proper error handling patterns
- API rate limiting considerations
- Network connectivity issues
- Invalid data handling
- Permission-based failures

## API Coverage

These examples cover the major Zendesk API endpoints:
- **Tickets**: `/tickets.json`, `/tickets/{id}.json`, `/search.json`
- **Users**: `/users.json`, `/users/{id}.json`, `/users/search.json`
- **Organizations**: `/organizations.json`, `/organizations/{id}.json`
- **Authentication**: All examples use API token authentication
- **Error Handling**: Comprehensive error scenarios

## Next Steps

After running these examples:
1. Review the test data created in your Zendesk instance
2. Modify the examples for your specific use cases
3. Implement proper error handling for production use
4. Consider rate limiting for bulk operations
5. Add logging and monitoring for production deployments