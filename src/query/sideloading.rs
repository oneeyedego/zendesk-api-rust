use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A response that contains the primary resource along with side-loaded related resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideloadedResponse<T> {
    /// The primary resources requested
    #[serde(flatten)]
    pub primary: T,

    /// Side-loaded related resources (users, organizations, etc.)
    #[serde(flatten)]
    pub sideloaded: HashMap<String, serde_json::Value>,

    /// Pagination information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

impl<T> SideloadedResponse<T> {
    /// Extract side-loaded users from the response
    pub fn users(&self) -> Option<Vec<crate::models::user::User>> {
        self.sideloaded
            .get("users")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Extract side-loaded organizations from the response
    pub fn organizations(&self) -> Option<Vec<crate::models::organization::Organization>> {
        self.sideloaded
            .get("organizations")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Extract side-loaded tickets from the response
    pub fn tickets(&self) -> Option<Vec<crate::models::ticket::Ticket>> {
        self.sideloaded
            .get("tickets")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Extract any side-loaded resource by name
    pub fn sideloaded_resource<U>(&self, resource_name: &str) -> Option<Vec<U>>
    where
        U: serde::de::DeserializeOwned,
    {
        self.sideloaded
            .get(resource_name)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Get a single side-loaded resource by name and ID
    pub fn find_sideloaded<U>(&self, resource_name: &str, id: u64) -> Option<U>
    where
        U: serde::de::DeserializeOwned + HasId,
    {
        self.sideloaded_resource::<U>(resource_name)?
            .into_iter()
            .find(|item| item.id() == Some(id))
    }

    /// Check if a specific resource type was side-loaded
    pub fn has_sideloaded(&self, resource_name: &str) -> bool {
        self.sideloaded.contains_key(resource_name)
    }

    /// Get all side-loaded resource names
    pub fn sideloaded_resource_names(&self) -> Vec<&String> {
        self.sideloaded.keys().collect()
    }
}

/// Trait for types that have an ID field for lookup purposes
pub trait HasId {
    fn id(&self) -> Option<u64>;
}

impl HasId for crate::models::user::User {
    fn id(&self) -> Option<u64> {
        self.id
    }
}

impl HasId for crate::models::organization::Organization {
    fn id(&self) -> Option<u64> {
        self.id
    }
}

impl HasId for crate::models::ticket::Ticket {
    fn id(&self) -> Option<u64> {
        self.id
    }
}

/// Specialized response types for common Zendesk resources with side-loading
pub type TicketsWithSideloading = SideloadedResponse<crate::models::ticket::TicketsResponse>;
pub type UsersWithSideloading = SideloadedResponse<crate::models::user::UsersResponse>;
pub type OrganizationsWithSideloading =
    SideloadedResponse<crate::models::organization::OrganizationsResponse>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sideloaded_response_deserialization() {
        let json_data = json!({
            "tickets": [
                {
                    "id": 1,
                    "subject": "Test ticket",
                    "requester_id": 100,
                    "organization_id": 200
                }
            ],
            "users": [
                {
                    "id": 100,
                    "name": "John Doe",
                    "email": "john@example.com"
                }
            ],
            "organizations": [
                {
                    "id": 200,
                    "name": "Acme Corp"
                }
            ],
            "count": 1,
            "next_page": null,
            "previous_page": null
        });

        let response: SideloadedResponse<crate::models::ticket::TicketsResponse> =
            serde_json::from_value(json_data).unwrap();

        // Check primary resource
        assert_eq!(response.primary.tickets.len(), 1);
        assert_eq!(response.primary.tickets[0].id, Some(1));

        // Check side-loaded users
        let users = response.users().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, Some(100));
        assert_eq!(users[0].name, "John Doe");

        // Check side-loaded organizations
        let orgs = response.organizations().unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].id, Some(200));
        assert_eq!(orgs[0].name, "Acme Corp");

        // Check metadata
        assert_eq!(response.count, Some(1));
        assert!(response.next_page.is_none());
    }

    #[test]
    fn test_has_sideloaded() {
        let json_data = json!({
            "tickets": [],
            "users": [],
            "count": 0
        });

        let response: SideloadedResponse<crate::models::ticket::TicketsResponse> =
            serde_json::from_value(json_data).unwrap();

        assert!(response.has_sideloaded("users"));
        assert!(!response.has_sideloaded("organizations"));
        assert!(!response.has_sideloaded("groups"));
    }

    #[test]
    fn test_sideloaded_resource_names() {
        let json_data = json!({
            "tickets": [],
            "users": [],
            "organizations": [],
            "count": 0
        });

        let response: SideloadedResponse<crate::models::ticket::TicketsResponse> =
            serde_json::from_value(json_data).unwrap();

        let mut resource_names: Vec<_> = response
            .sideloaded_resource_names()
            .into_iter()
            .map(|s| s.as_str())
            .collect();
        resource_names.sort();

        assert_eq!(resource_names, vec!["organizations", "users"]);
    }
}
