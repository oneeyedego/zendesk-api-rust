use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    
    pub subject: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<TicketType>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    
    pub body: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub id: u64,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    New,
    Open,
    Pending,
    Hold,
    Solved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketType {
    Problem,
    Incident,
    Question,
    Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCreateRequest {
    pub ticket: TicketCreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCreate {
    pub subject: String,
    pub comment: TicketComment,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<TicketType>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketResponse {
    pub ticket: Ticket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketsResponse {
    pub tickets: Vec<Ticket>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

impl Ticket {
    pub fn builder(subject: impl Into<String>) -> TicketBuilder {
        TicketBuilder::new(subject)
    }
}

#[derive(Debug)]
pub struct TicketBuilder {
    ticket: TicketCreate,
}

impl TicketBuilder {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            ticket: TicketCreate {
                subject: subject.into(),
                comment: TicketComment {
                    id: None,
                    body: String::new(),
                    author_id: None,
                    created_at: None,
                    public: Some(true),
                    html_body: None,
                },
                priority: None,
                ticket_type: None,
                status: None,
                requester_id: None,
                assignee_id: None,
                group_id: None,
                tags: None,
            },
        }
    }
    
    pub fn comment(mut self, body: impl Into<String>) -> Self {
        self.ticket.comment.body = body.into();
        self
    }
    
    pub fn priority(mut self, priority: TicketPriority) -> Self {
        self.ticket.priority = Some(priority);
        self
    }
    
    pub fn ticket_type(mut self, ticket_type: TicketType) -> Self {
        self.ticket.ticket_type = Some(ticket_type);
        self
    }
    
    pub fn status(mut self, status: TicketStatus) -> Self {
        self.ticket.status = Some(status);
        self
    }
    
    pub fn requester_id(mut self, requester_id: u64) -> Self {
        self.ticket.requester_id = Some(requester_id);
        self
    }
    
    pub fn assignee_id(mut self, assignee_id: u64) -> Self {
        self.ticket.assignee_id = Some(assignee_id);
        self
    }
    
    pub fn group_id(mut self, group_id: u64) -> Self {
        self.ticket.group_id = Some(group_id);
        self
    }
    
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.ticket.tags = Some(tags);
        self
    }
    
    pub fn build(self) -> TicketCreateRequest {
        TicketCreateRequest {
            ticket: self.ticket,
        }
    }
}