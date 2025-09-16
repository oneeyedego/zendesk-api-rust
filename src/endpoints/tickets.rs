use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::ticket::{
    Ticket, TicketCreateRequest, TicketResponse, TicketsResponse
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
    
    pub async fn update_ticket(&self, ticket_id: u64, ticket_request: TicketCreateRequest) -> Result<Ticket> {
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
}