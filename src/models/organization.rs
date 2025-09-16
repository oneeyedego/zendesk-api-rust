use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationCreateRequest {
    pub organization: OrganizationCreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationCreate {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub organization: Organization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationsResponse {
    pub organizations: Vec<Organization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

impl Organization {
    pub fn builder(name: impl Into<String>) -> OrganizationBuilder {
        OrganizationBuilder::new(name)
    }
}

#[derive(Debug)]
pub struct OrganizationBuilder {
    organization: OrganizationCreate,
}

impl OrganizationBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            organization: OrganizationCreate {
                name: name.into(),
                details: None,
                notes: None,
                domain_names: None,
                tags: None,
                external_id: None,
            },
        }
    }

    pub fn details(mut self, details: impl Into<String>) -> Self {
        self.organization.details = Some(details.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.organization.notes = Some(notes.into());
        self
    }

    pub fn domain_names(mut self, domain_names: Vec<String>) -> Self {
        self.organization.domain_names = Some(domain_names);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.organization.tags = Some(tags);
        self
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.organization.external_id = Some(external_id.into());
        self
    }

    pub fn build(self) -> OrganizationCreateRequest {
        OrganizationCreateRequest {
            organization: self.organization,
        }
    }
}
