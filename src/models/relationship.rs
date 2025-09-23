use serde::{Deserialize, Serialize};

/// Represents the type of Zendesk object that can participate in relationships
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ZendeskObjectType {
    #[serde(rename = "zen:user")]
    User,
    #[serde(rename = "zen:organization")]
    Organization,
    #[serde(rename = "zen:ticket")]
    Ticket,
    #[serde(rename = "zen:group")]
    Group,
    #[serde(untagged)]
    CustomObject(String), // For custom objects like "zen:custom_object:apartment"
}

impl ZendeskObjectType {
    pub fn custom_object(key: &str) -> Self {
        Self::CustomObject(format!("zen:custom_object:{}", key))
    }

    pub fn as_api_string(&self) -> String {
        match self {
            Self::User => "zen:user".to_string(),
            Self::Organization => "zen:organization".to_string(),
            Self::Ticket => "zen:ticket".to_string(),
            Self::Group => "zen:group".to_string(),
            Self::CustomObject(s) => s.clone(),
        }
    }
}

impl std::fmt::Display for ZendeskObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_api_string())
    }
}

/// Represents a lookup relationship field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupRelationshipField {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub active: bool,
    pub required: bool,
    #[serde(rename = "type")]
    pub field_type: String, // Should be "lookup"
    pub relationship_target_type: ZendeskObjectType,
    pub relationship_filter: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub key: Option<String>,
    pub raw_title: Option<String>,
    pub raw_description: Option<String>,
}

/// Request to create a lookup relationship field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLookupRelationshipField {
    pub title: String,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub field_type: String, // Must be "lookup"
    pub relationship_target_type: ZendeskObjectType,
    pub relationship_filter: Option<serde_json::Value>,
    pub key: Option<String>,
}

/// Response when querying sources by target in a lookup relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipSourcesResponse<T> {
    pub results: Vec<T>,
    pub count: u64,
    pub next_page: Option<String>,
    pub previous_page: Option<String>,
    pub meta: Option<RelationshipMeta>,
}

/// Metadata for relationship queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipMeta {
    pub has_more: bool,
    pub after_cursor: Option<String>,
    pub before_cursor: Option<String>,
}

/// A single relationship record linking source to target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRecord {
    pub source_id: u64,
    pub source_type: ZendeskObjectType,
    pub target_id: u64,
    pub target_type: ZendeskObjectType,
    pub field_id: u64,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to create or update a relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRequest {
    pub source_id: u64,
    pub source_type: ZendeskObjectType,
    pub target_id: u64,
    pub target_type: ZendeskObjectType,
    pub field_id: u64,
}

/// Response containing relationship field definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipFieldsResponse {
    pub fields: Vec<LookupRelationshipField>,
    pub count: u64,
}

/// Parameters for querying relationships
#[derive(Debug, Clone, Default)]
pub struct RelationshipQueryParams {
    pub target_type: Option<ZendeskObjectType>,
    pub target_id: Option<u64>,
    pub source_type: Option<ZendeskObjectType>,
    pub source_id: Option<u64>,
    pub field_id: Option<u64>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl RelationshipQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target(mut self, target_type: ZendeskObjectType, target_id: u64) -> Self {
        self.target_type = Some(target_type);
        self.target_id = Some(target_id);
        self
    }

    pub fn source(mut self, source_type: ZendeskObjectType, source_id: u64) -> Self {
        self.source_type = Some(source_type);
        self.source_id = Some(source_id);
        self
    }

    pub fn field_id(mut self, field_id: u64) -> Self {
        self.field_id = Some(field_id);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }
}

/// Builder for creating lookup relationship fields
#[derive(Debug)]
pub struct LookupRelationshipFieldBuilder {
    field: CreateLookupRelationshipField,
}

impl LookupRelationshipFieldBuilder {
    pub fn new(title: impl Into<String>, target_type: ZendeskObjectType) -> Self {
        Self {
            field: CreateLookupRelationshipField {
                title: title.into(),
                description: None,
                active: Some(true),
                required: Some(false),
                field_type: "lookup".to_string(),
                relationship_target_type: target_type,
                relationship_filter: None,
                key: None,
            },
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.field.description = Some(description.into());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.field.required = Some(required);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.field.active = Some(active);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.field.key = Some(key.into());
        self
    }

    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.field.relationship_filter = Some(filter);
        self
    }

    /// Add a filter to show only users with a specific role
    pub fn filter_users_by_role(mut self, role: &str) -> Self {
        let filter = serde_json::json!({
            "all": [
                {
                    "field": "role",
                    "operator": "is",
                    "value": role
                }
            ]
        });
        self.field.relationship_filter = Some(filter);
        self
    }

    /// Add a filter to show only active records
    pub fn filter_active_only(mut self) -> Self {
        let filter = serde_json::json!({
            "all": [
                {
                    "field": "active",
                    "operator": "is",
                    "value": true
                }
            ]
        });
        self.field.relationship_filter = Some(filter);
        self
    }

    pub fn build(self) -> CreateLookupRelationshipField {
        self.field
    }
}

impl CreateLookupRelationshipField {
    pub fn builder(
        title: impl Into<String>,
        target_type: ZendeskObjectType,
    ) -> LookupRelationshipFieldBuilder {
        LookupRelationshipFieldBuilder::new(title, target_type)
    }

    /// Quick constructor for a user lookup field
    pub fn user_lookup(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            active: Some(true),
            required: Some(false),
            field_type: "lookup".to_string(),
            relationship_target_type: ZendeskObjectType::User,
            relationship_filter: None,
            key: None,
        }
    }

    /// Quick constructor for an organization lookup field
    pub fn organization_lookup(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            active: Some(true),
            required: Some(false),
            field_type: "lookup".to_string(),
            relationship_target_type: ZendeskObjectType::Organization,
            relationship_filter: None,
            key: None,
        }
    }

    /// Quick constructor for a ticket lookup field
    pub fn ticket_lookup(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            active: Some(true),
            required: Some(false),
            field_type: "lookup".to_string(),
            relationship_target_type: ZendeskObjectType::Ticket,
            relationship_filter: None,
            key: None,
        }
    }

    /// Quick constructor for a custom object lookup field
    pub fn custom_object_lookup(title: impl Into<String>, custom_object_key: &str) -> Self {
        Self {
            title: title.into(),
            description: None,
            active: Some(true),
            required: Some(false),
            field_type: "lookup".to_string(),
            relationship_target_type: ZendeskObjectType::custom_object(custom_object_key),
            relationship_filter: None,
            key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zendesk_object_type_serialization() {
        let user_type = ZendeskObjectType::User;
        let serialized = serde_json::to_string(&user_type).unwrap();
        assert_eq!(serialized, "\"zen:user\"");

        let custom_type = ZendeskObjectType::custom_object("apartment");
        let serialized = serde_json::to_string(&custom_type).unwrap();
        assert_eq!(serialized, "\"zen:custom_object:apartment\"");
    }

    #[test]
    fn test_zendesk_object_type_deserialization() {
        let json = "\"zen:user\"";
        let deserialized: ZendeskObjectType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, ZendeskObjectType::User);

        let json = "\"zen:custom_object:apartment\"";
        let deserialized: ZendeskObjectType = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, ZendeskObjectType::custom_object("apartment"));
    }

    #[test]
    fn test_lookup_relationship_field_builder() {
        let field =
            CreateLookupRelationshipField::builder("Account Manager", ZendeskObjectType::User)
                .description("The account manager for this organization")
                .required(true)
                .filter_users_by_role("agent")
                .build();

        assert_eq!(field.title, "Account Manager");
        assert_eq!(
            field.description,
            Some("The account manager for this organization".to_string())
        );
        assert_eq!(field.required, Some(true));
        assert_eq!(field.relationship_target_type, ZendeskObjectType::User);
        assert!(field.relationship_filter.is_some());
    }

    #[test]
    fn test_quick_constructors() {
        let user_lookup = CreateLookupRelationshipField::user_lookup("Success Manager");
        assert_eq!(user_lookup.title, "Success Manager");
        assert_eq!(
            user_lookup.relationship_target_type,
            ZendeskObjectType::User
        );
        assert_eq!(user_lookup.field_type, "lookup");

        let custom_lookup =
            CreateLookupRelationshipField::custom_object_lookup("Related Product", "product");
        assert_eq!(custom_lookup.title, "Related Product");
        assert_eq!(
            custom_lookup.relationship_target_type,
            ZendeskObjectType::custom_object("product")
        );
    }

    #[test]
    fn test_relationship_query_params() {
        let params = RelationshipQueryParams::new()
            .target(ZendeskObjectType::User, 12345)
            .source(ZendeskObjectType::Ticket, 0) // 0 means we want all tickets
            .field_id(67890)
            .per_page(50);

        assert_eq!(params.target_type, Some(ZendeskObjectType::User));
        assert_eq!(params.target_id, Some(12345));
        assert_eq!(params.field_id, Some(67890));
        assert_eq!(params.per_page, Some(50));
    }
}
