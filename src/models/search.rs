use crate::models::{organization::Organization, ticket::Ticket, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "result_type")]
pub enum SearchResult {
    #[serde(rename = "ticket")]
    Ticket(Ticket),

    #[serde(rename = "user")]
    User(User),

    #[serde(rename = "organization")]
    Organization(Organization),

    #[serde(rename = "group")]
    Group(Group),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub default: Option<bool>,
    pub deleted: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCountResponse {
    pub count: SearchCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCount {
    pub value: u64,
    pub refreshed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchExportResponse {
    pub results: Vec<SearchResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_stream: Option<bool>,
}

// Search query builder utilities
#[derive(Debug, Clone)]
pub struct SearchQueryBuilder {
    query_parts: Vec<String>,
}

impl SearchQueryBuilder {
    pub fn new() -> Self {
        Self {
            query_parts: Vec::new(),
        }
    }

    // Resource type filters
    pub fn tickets(mut self) -> Self {
        self.query_parts.push("type:ticket".to_string());
        self
    }

    pub fn users(mut self) -> Self {
        self.query_parts.push("type:user".to_string());
        self
    }

    pub fn organizations(mut self) -> Self {
        self.query_parts.push("type:organization".to_string());
        self
    }

    pub fn groups(mut self) -> Self {
        self.query_parts.push("type:group".to_string());
        self
    }

    // Ticket-specific filters
    pub fn status(mut self, status: &str) -> Self {
        self.query_parts.push(format!("status:{}", status));
        self
    }

    pub fn priority(mut self, priority: &str) -> Self {
        self.query_parts.push(format!("priority:{}", priority));
        self
    }

    pub fn ticket_type(mut self, ticket_type: &str) -> Self {
        self.query_parts
            .push(format!("ticket_type:{}", ticket_type));
        self
    }

    pub fn assignee_id(mut self, assignee_id: u64) -> Self {
        self.query_parts.push(format!("assignee:{}", assignee_id));
        self
    }

    pub fn requester_id(mut self, requester_id: u64) -> Self {
        self.query_parts.push(format!("requester:{}", requester_id));
        self
    }

    pub fn organization_id(mut self, org_id: u64) -> Self {
        self.query_parts.push(format!("organization:{}", org_id));
        self
    }

    pub fn group_id(mut self, group_id: u64) -> Self {
        self.query_parts.push(format!("group:{}", group_id));
        self
    }

    pub fn tags(mut self, tag: &str) -> Self {
        self.query_parts.push(format!("tags:{}", tag));
        self
    }

    // Date filters
    pub fn created_after(mut self, date: &str) -> Self {
        self.query_parts.push(format!("created>{}", date));
        self
    }

    pub fn created_before(mut self, date: &str) -> Self {
        self.query_parts.push(format!("created<{}", date));
        self
    }

    pub fn updated_after(mut self, date: &str) -> Self {
        self.query_parts.push(format!("updated>{}", date));
        self
    }

    pub fn updated_before(mut self, date: &str) -> Self {
        self.query_parts.push(format!("updated<{}", date));
        self
    }

    // Text search
    pub fn text(mut self, text: &str) -> Self {
        // Escape quotes and wrap in quotes if it contains spaces
        let escaped_text = if text.contains(' ') {
            format!("\"{}\"", text.replace('"', "\\\""))
        } else {
            text.to_string()
        };
        self.query_parts.push(escaped_text);
        self
    }

    pub fn subject_contains(mut self, text: &str) -> Self {
        self.query_parts
            .push(format!("subject:\"{}\"", text.replace('"', "\\\"")));
        self
    }

    pub fn description_contains(mut self, text: &str) -> Self {
        self.query_parts
            .push(format!("description:\"{}\"", text.replace('"', "\\\"")));
        self
    }

    // Custom fields
    pub fn custom_field(mut self, field_id: u64, value: &str) -> Self {
        self.query_parts
            .push(format!("custom_field_{}:{}", field_id, value));
        self
    }

    // Raw query part
    pub fn raw(mut self, query_part: &str) -> Self {
        self.query_parts.push(query_part.to_string());
        self
    }

    pub fn build(self) -> String {
        self.query_parts.join(" ")
    }
}

impl Default for SearchQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Search sort options
#[derive(Debug, Clone, Serialize)]
pub enum SearchSortBy {
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "ticket_type")]
    TicketType,
}

#[derive(Debug, Clone, Serialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl std::fmt::Display for SearchSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchSortBy::UpdatedAt => write!(f, "updated_at"),
            SearchSortBy::CreatedAt => write!(f, "created_at"),
            SearchSortBy::Priority => write!(f, "priority"),
            SearchSortBy::Status => write!(f, "status"),
            SearchSortBy::TicketType => write!(f, "ticket_type"),
        }
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "asc"),
            SortOrder::Descending => write!(f, "desc"),
        }
    }
}
