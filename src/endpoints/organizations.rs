use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::organization::{
    Organization, OrganizationCreateRequest, OrganizationResponse, OrganizationsResponse
};

impl ZendeskClient {
    pub async fn create_organization(&self, org_request: OrganizationCreateRequest) -> Result<Organization> {
        let response: OrganizationResponse = self.post("organizations.json", &org_request).await?;
        Ok(response.organization)
    }
    
    pub async fn get_organization(&self, organization_id: u64) -> Result<Organization> {
        let endpoint = format!("organizations/{}.json", organization_id);
        let response: OrganizationResponse = self.get(&endpoint).await?;
        Ok(response.organization)
    }
    
    pub async fn update_organization(&self, organization_id: u64, org_request: OrganizationCreateRequest) -> Result<Organization> {
        let endpoint = format!("organizations/{}.json", organization_id);
        let response: OrganizationResponse = self.put(&endpoint, &org_request).await?;
        Ok(response.organization)
    }
    
    pub async fn delete_organization(&self, organization_id: u64) -> Result<()> {
        let endpoint = format!("organizations/{}.json", organization_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }
    
    pub async fn list_organizations(&self) -> Result<Vec<Organization>> {
        let response: OrganizationsResponse = self.get("organizations.json").await?;
        Ok(response.organizations)
    }
    
    pub async fn search_organizations(&self, query: &str) -> Result<Vec<Organization>> {
        let endpoint = format!("organizations/search.json?query={}", query);
        let response: OrganizationsResponse = self.get(&endpoint).await?;
        Ok(response.organizations)
    }
}