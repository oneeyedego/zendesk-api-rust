use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::ticket::{
    Ticket, TicketComment, TicketCommentCountResponse, TicketCommentsResponse, TicketCreateRequest,
    TicketResponse, TicketsResponse,
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
}
