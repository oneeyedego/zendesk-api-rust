use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::relationship::{
    CreateLookupRelationshipField, LookupRelationshipField, RelationshipSourcesResponse,
    ZendeskObjectType,
};
use crate::query::QueryParams;
use crate::query::SideloadedResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupRelationshipFieldResponse {
    pub ticket_field: Option<LookupRelationshipField>,
    pub user_field: Option<LookupRelationshipField>,
    pub organization_field: Option<LookupRelationshipField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupRelationshipFieldsResponse {
    pub ticket_fields: Option<Vec<LookupRelationshipField>>,
    pub user_fields: Option<Vec<LookupRelationshipField>>,
    pub organization_fields: Option<Vec<LookupRelationshipField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketLookupFieldRequest {
    pub ticket_field: CreateLookupRelationshipField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserLookupFieldRequest {
    pub user_field: CreateLookupRelationshipField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationLookupFieldRequest {
    pub organization_field: CreateLookupRelationshipField,
}

impl ZendeskClient {
    /// Get sources by target for lookup relationships
    /// This is the main method for traversing lookup relationships
    pub async fn get_sources_by_target<T>(
        &self,
        target_type: &ZendeskObjectType,
        target_id: u64,
        field_id: u64,
        source_type: &ZendeskObjectType,
    ) -> Result<RelationshipSourcesResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!(
            "api/v2/{}/{}/relationship_fields/{}/{}",
            target_type.as_api_string(),
            target_id,
            field_id,
            source_type.as_api_string()
        );
        self.get(&endpoint).await
    }

    /// Get sources by target with side-loading support
    pub async fn get_sources_by_target_with_sideloading<T>(
        &self,
        target_type: &ZendeskObjectType,
        target_id: u64,
        field_id: u64,
        source_type: &ZendeskObjectType,
        include: &[&str],
    ) -> Result<SideloadedResponse<RelationshipSourcesResponse<T>>>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!(
            "api/v2/{}/{}/relationship_fields/{}/{}",
            target_type.as_api_string(),
            target_id,
            field_id,
            source_type.as_api_string()
        );
        self.get_with_sideloading(&endpoint, include).await
    }

    /// Get sources by target with query parameters (pagination, etc.)
    pub async fn get_sources_by_target_with_params<T>(
        &self,
        target_type: &ZendeskObjectType,
        target_id: u64,
        field_id: u64,
        source_type: &ZendeskObjectType,
        params: &QueryParams,
    ) -> Result<RelationshipSourcesResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!(
            "api/v2/{}/{}/relationship_fields/{}/{}",
            target_type.as_api_string(),
            target_id,
            field_id,
            source_type.as_api_string()
        );
        self.get_with_params(&endpoint, params).await
    }

    /// Create a lookup relationship field for tickets
    pub async fn create_ticket_lookup_field(
        &self,
        field: CreateLookupRelationshipField,
    ) -> Result<LookupRelationshipField> {
        let request = CreateTicketLookupFieldRequest {
            ticket_field: field,
        };
        let response: LookupRelationshipFieldResponse =
            self.post("api/v2/ticket_fields.json", &request).await?;
        response.ticket_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse(
                "No ticket field in response".to_string(),
            )
        })
    }

    /// Create a lookup relationship field for users
    pub async fn create_user_lookup_field(
        &self,
        field: CreateLookupRelationshipField,
    ) -> Result<LookupRelationshipField> {
        let request = CreateUserLookupFieldRequest { user_field: field };
        let response: LookupRelationshipFieldResponse =
            self.post("api/v2/user_fields.json", &request).await?;
        response.user_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse("No user field in response".to_string())
        })
    }

    /// Create a lookup relationship field for organizations
    pub async fn create_organization_lookup_field(
        &self,
        field: CreateLookupRelationshipField,
    ) -> Result<LookupRelationshipField> {
        let request = CreateOrganizationLookupFieldRequest {
            organization_field: field,
        };
        let response: LookupRelationshipFieldResponse = self
            .post("api/v2/organization_fields.json", &request)
            .await?;
        response.organization_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse(
                "No organization field in response".to_string(),
            )
        })
    }

    /// List all lookup relationship fields for tickets
    pub async fn list_ticket_lookup_fields(&self) -> Result<Vec<LookupRelationshipField>> {
        let response: LookupRelationshipFieldsResponse =
            self.get("api/v2/ticket_fields.json").await?;
        Ok(response
            .ticket_fields
            .unwrap_or_default()
            .into_iter()
            .filter(|field| field.field_type == "lookup")
            .collect())
    }

    /// List all lookup relationship fields for users
    pub async fn list_user_lookup_fields(&self) -> Result<Vec<LookupRelationshipField>> {
        let response: LookupRelationshipFieldsResponse =
            self.get("api/v2/user_fields.json").await?;
        Ok(response
            .user_fields
            .unwrap_or_default()
            .into_iter()
            .filter(|field| field.field_type == "lookup")
            .collect())
    }

    /// List all lookup relationship fields for organizations
    pub async fn list_organization_lookup_fields(&self) -> Result<Vec<LookupRelationshipField>> {
        let response: LookupRelationshipFieldsResponse =
            self.get("api/v2/organization_fields.json").await?;
        Ok(response
            .organization_fields
            .unwrap_or_default()
            .into_iter()
            .filter(|field| field.field_type == "lookup")
            .collect())
    }

    /// Get a specific ticket lookup field
    pub async fn get_ticket_lookup_field(&self, field_id: u64) -> Result<LookupRelationshipField> {
        let endpoint = format!("api/v2/ticket_fields/{}.json", field_id);
        let response: LookupRelationshipFieldResponse = self.get(&endpoint).await?;
        response.ticket_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse(
                "No ticket field in response".to_string(),
            )
        })
    }

    /// Get a specific user lookup field
    pub async fn get_user_lookup_field(&self, field_id: u64) -> Result<LookupRelationshipField> {
        let endpoint = format!("api/v2/user_fields/{}.json", field_id);
        let response: LookupRelationshipFieldResponse = self.get(&endpoint).await?;
        response.user_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse("No user field in response".to_string())
        })
    }

    /// Get a specific organization lookup field
    pub async fn get_organization_lookup_field(
        &self,
        field_id: u64,
    ) -> Result<LookupRelationshipField> {
        let endpoint = format!("api/v2/organization_fields/{}.json", field_id);
        let response: LookupRelationshipFieldResponse = self.get(&endpoint).await?;
        response.organization_field.ok_or_else(|| {
            crate::errors::ZendeskError::UnexpectedResponse(
                "No organization field in response".to_string(),
            )
        })
    }

    /// Helper method to get all tickets related to a specific user via a lookup field
    pub async fn get_tickets_related_to_user(
        &self,
        user_id: u64,
        lookup_field_id: u64,
    ) -> Result<RelationshipSourcesResponse<crate::models::ticket::Ticket>> {
        self.get_sources_by_target(
            &ZendeskObjectType::User,
            user_id,
            lookup_field_id,
            &ZendeskObjectType::Ticket,
        )
        .await
    }

    /// Helper method to get all tickets related to a specific organization via a lookup field
    pub async fn get_tickets_related_to_organization(
        &self,
        org_id: u64,
        lookup_field_id: u64,
    ) -> Result<RelationshipSourcesResponse<crate::models::ticket::Ticket>> {
        self.get_sources_by_target(
            &ZendeskObjectType::Organization,
            org_id,
            lookup_field_id,
            &ZendeskObjectType::Ticket,
        )
        .await
    }

    /// Helper method to get all users related to a specific ticket via a lookup field
    pub async fn get_users_related_to_ticket(
        &self,
        ticket_id: u64,
        lookup_field_id: u64,
    ) -> Result<RelationshipSourcesResponse<crate::models::user::User>> {
        self.get_sources_by_target(
            &ZendeskObjectType::Ticket,
            ticket_id,
            lookup_field_id,
            &ZendeskObjectType::User,
        )
        .await
    }

    /// Helper method to get all organizations related to a specific user via a lookup field
    pub async fn get_organizations_related_to_user(
        &self,
        user_id: u64,
        lookup_field_id: u64,
    ) -> Result<RelationshipSourcesResponse<crate::models::organization::Organization>> {
        self.get_sources_by_target(
            &ZendeskObjectType::User,
            user_id,
            lookup_field_id,
            &ZendeskObjectType::Organization,
        )
        .await
    }

    /// Advanced helper: Get all tickets with their related resources via lookup relationships
    pub async fn get_tickets_with_lookup_relationships(
        &self,
        ticket_ids: &[u64],
        lookup_fields: &[(u64, ZendeskObjectType)], // (field_id, target_type)
    ) -> Result<
        Vec<(
            crate::models::ticket::Ticket,
            std::collections::HashMap<u64, Vec<serde_json::Value>>,
        )>,
    > {
        let mut results = Vec::new();

        for ticket_id in ticket_ids {
            // Get the base ticket first
            let ticket = self.get_ticket(*ticket_id).await?;
            let mut relationships = std::collections::HashMap::new();

            // For each lookup field, get the related resources
            for (field_id, target_type) in lookup_fields {
                match target_type {
                    ZendeskObjectType::User => {
                        let users: RelationshipSourcesResponse<crate::models::user::User> = self
                            .get_sources_by_target(
                                &ZendeskObjectType::Ticket,
                                *ticket_id,
                                *field_id,
                                target_type,
                            )
                            .await?;
                        let user_values: Vec<serde_json::Value> = users
                            .results
                            .into_iter()
                            .map(|u| serde_json::to_value(u).unwrap_or_default())
                            .collect();
                        relationships.insert(*field_id, user_values);
                    }
                    ZendeskObjectType::Organization => {
                        let orgs: RelationshipSourcesResponse<
                            crate::models::organization::Organization,
                        > = self
                            .get_sources_by_target(
                                &ZendeskObjectType::Ticket,
                                *ticket_id,
                                *field_id,
                                target_type,
                            )
                            .await?;
                        let org_values: Vec<serde_json::Value> = orgs
                            .results
                            .into_iter()
                            .map(|o| serde_json::to_value(o).unwrap_or_default())
                            .collect();
                        relationships.insert(*field_id, org_values);
                    }
                    _ => {
                        // For custom objects or other types, return raw JSON
                        let raw: RelationshipSourcesResponse<serde_json::Value> = self
                            .get_sources_by_target(
                                &ZendeskObjectType::Ticket,
                                *ticket_id,
                                *field_id,
                                target_type,
                            )
                            .await?;
                        relationships.insert(*field_id, raw.results);
                    }
                }
            }

            results.push((ticket, relationships));
        }

        Ok(results)
    }

    /// Delete a lookup relationship field
    pub async fn delete_ticket_lookup_field(&self, field_id: u64) -> Result<()> {
        let endpoint = format!("api/v2/ticket_fields/{}.json", field_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    /// Delete a user lookup relationship field
    pub async fn delete_user_lookup_field(&self, field_id: u64) -> Result<()> {
        let endpoint = format!("api/v2/user_fields/{}.json", field_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    /// Delete an organization lookup relationship field
    pub async fn delete_organization_lookup_field(&self, field_id: u64) -> Result<()> {
        let endpoint = format!("api/v2/organization_fields/{}.json", field_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }
}
