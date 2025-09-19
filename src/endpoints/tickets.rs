use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::ticket::{
    Ticket, TicketComment, TicketCommentCountResponse, TicketCommentCreate, TicketCommentRequest,
    TicketCommentsResponse, TicketCreateRequest, TicketResponse, TicketsResponse,
};

impl ZendeskClient {
    pub async fn create_ticket(&self, ticket_request: TicketCreateRequest) -> Result<Ticket> {
        let response: TicketResponse = self.post("tickets.json", &ticket_request).await?;
        Ok(response.ticket)
    }

    pub async fn get_ticket(&self, ticket_id: u64) -> Result<Ticket> {
        let endpoint = format!("tickets/{}.json", ticket_id);
        let response: TicketResponse = self.get(&endpoint).await?;
        Ok(response.ticket)
    }

    pub async fn update_ticket(
        &self,
        ticket_id: u64,
        ticket_request: TicketCreateRequest,
    ) -> Result<Ticket> {
        let endpoint = format!("tickets/{}.json", ticket_id);
        let response: TicketResponse = self.put(&endpoint, &ticket_request).await?;
        Ok(response.ticket)
    }

    pub async fn delete_ticket(&self, ticket_id: u64) -> Result<()> {
        let endpoint = format!("tickets/{}.json", ticket_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let response: TicketsResponse = self.get("tickets.json").await?;
        Ok(response.tickets)
    }

    pub async fn list_tickets_assigned_to(&self, assignee_id: u64) -> Result<Vec<Ticket>> {
        let endpoint = format!("users/{}/tickets/assigned.json", assignee_id);
        let response: TicketsResponse = self.get(&endpoint).await?;
        Ok(response.tickets)
    }

    pub async fn list_tickets_requested_by(&self, requester_id: u64) -> Result<Vec<Ticket>> {
        let endpoint = format!("users/{}/tickets/requested.json", requester_id);
        let response: TicketsResponse = self.get(&endpoint).await?;
        Ok(response.tickets)
    }

    pub async fn search_tickets(&self, query: &str) -> Result<Vec<Ticket>> {
        let endpoint = format!("search.json?query=type:ticket {}", query);
        let response: TicketsResponse = self.get(&endpoint).await?;
        Ok(response.tickets)
    }

    // Ticket Comments API
    pub async fn get_ticket_comments(&self, ticket_id: u64) -> Result<Vec<TicketComment>> {
        let endpoint = format!("tickets/{}/comments.json", ticket_id);
        let response: TicketCommentsResponse = self.get(&endpoint).await?;
        Ok(response.comments)
    }

    pub async fn get_ticket_comments_with_pagination(
        &self,
        ticket_id: u64,
        page: Option<&str>,
    ) -> Result<TicketCommentsResponse> {
        let endpoint = match page {
            Some(page_url) => {
                // Extract just the path and query from the full URL
                if let Some(path_start) = page_url.find("/api/v2/") {
                    &page_url[path_start + 8..] // Skip "/api/v2/"
                } else {
                    return Err(crate::errors::ZendeskError::InvalidUrl(
                        page_url.to_string(),
                    ));
                }
            }
            None => &format!("tickets/{}/comments.json", ticket_id),
        };
        self.get(endpoint).await
    }

    pub async fn count_ticket_comments(&self, ticket_id: u64) -> Result<u64> {
        let endpoint = format!("tickets/{}/comments/count.json", ticket_id);
        let response: TicketCommentCountResponse = self.get(&endpoint).await?;
        Ok(response.count.value)
    }

    pub async fn make_comment_private(
        &self,
        ticket_id: u64,
        comment_id: u64,
    ) -> Result<TicketComment> {
        let endpoint = format!(
            "tickets/{}/comments/{}/make_private.json",
            ticket_id, comment_id
        );
        let response: serde_json::Value = self.put(&endpoint, &serde_json::json!({})).await?;

        // The API returns the updated comment in a different format, extract it
        if let Some(comment) = response.get("comment") {
            Ok(serde_json::from_value(comment.clone())?)
        } else {
            Err(crate::errors::ZendeskError::UnexpectedResponse(
                "Comment not found in response".to_string(),
            ))
        }
    }

    // Comment posting methods

    /// Add a comment to an existing ticket
    pub async fn add_ticket_comment(
        &self,
        ticket_id: u64,
        comment_request: TicketCommentRequest,
    ) -> Result<Ticket> {
        let endpoint = format!("tickets/{}.json", ticket_id);
        let response: TicketResponse = self.put(&endpoint, &comment_request).await?;
        Ok(response.ticket)
    }

    /// Add a public response (visible to requester) to a ticket
    pub async fn add_public_response(
        &self,
        ticket_id: u64,
        body: impl Into<String>,
    ) -> Result<Ticket> {
        let comment = TicketCommentCreate::public_response(body);
        let request = TicketCommentRequest {
            ticket: crate::models::ticket::TicketUpdate {
                comment,
                status: None,
                priority: None,
                assignee_id: None,
                group_id: None,
                tags: None,
            },
        };
        self.add_ticket_comment(ticket_id, request).await
    }

    /// Add a work note (internal note, not visible to requester) to a ticket
    pub async fn add_work_note(&self, ticket_id: u64, body: impl Into<String>) -> Result<Ticket> {
        let comment = TicketCommentCreate::work_note(body);
        let request = TicketCommentRequest {
            ticket: crate::models::ticket::TicketUpdate {
                comment,
                status: None,
                priority: None,
                assignee_id: None,
                group_id: None,
                tags: None,
            },
        };
        self.add_ticket_comment(ticket_id, request).await
    }

    /// Add a comment with optional ticket updates (status, assignee, tags, etc.)
    pub async fn add_comment_with_updates(
        &self,
        ticket_id: u64,
        comment: TicketCommentCreate,
        status: Option<crate::models::ticket::TicketStatus>,
        assignee_id: Option<u64>,
        tags: Option<Vec<String>>,
    ) -> Result<Ticket> {
        let request = TicketCommentRequest {
            ticket: crate::models::ticket::TicketUpdate {
                comment,
                status,
                priority: None,
                assignee_id,
                group_id: None,
                tags,
            },
        };
        self.add_ticket_comment(ticket_id, request).await
    }

    /// Solve ticket with a public response
    pub async fn solve_ticket_with_response(
        &self,
        ticket_id: u64,
        body: impl Into<String>,
    ) -> Result<Ticket> {
        let comment = TicketCommentCreate::public_response(body);
        let request = TicketCommentRequest {
            ticket: crate::models::ticket::TicketUpdate {
                comment,
                status: Some(crate::models::ticket::TicketStatus::Solved),
                priority: None,
                assignee_id: None,
                group_id: None,
                tags: None,
            },
        };
        self.add_ticket_comment(ticket_id, request).await
    }

    /// Change assignee with a work note
    pub async fn reassign_ticket_with_note(
        &self,
        ticket_id: u64,
        new_assignee_id: u64,
        note: impl Into<String>,
    ) -> Result<Ticket> {
        let comment = TicketCommentCreate::work_note(note);
        let request = TicketCommentRequest {
            ticket: crate::models::ticket::TicketUpdate {
                comment,
                status: None,
                priority: None,
                assignee_id: Some(new_assignee_id),
                group_id: None,
                tags: None,
            },
        };
        self.add_ticket_comment(ticket_id, request).await
    }
}
