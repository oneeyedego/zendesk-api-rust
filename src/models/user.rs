use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    
    pub name: String,
    pub email: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "end-user")]
    EndUser,
    Agent,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub user: UserCreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

impl User {
    pub fn builder(name: impl Into<String>, email: impl Into<String>) -> UserBuilder {
        UserBuilder::new(name, email)
    }
}

#[derive(Debug)]
pub struct UserBuilder {
    user: UserCreate,
}

impl UserBuilder {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            user: UserCreate {
                name: name.into(),
                email: email.into(),
                role: None,
                organization_id: None,
                phone: None,
                notes: None,
                tags: None,
                time_zone: None,
                locale: None,
            },
        }
    }
    
    pub fn role(mut self, role: UserRole) -> Self {
        self.user.role = Some(role);
        self
    }
    
    pub fn organization_id(mut self, organization_id: u64) -> Self {
        self.user.organization_id = Some(organization_id);
        self
    }
    
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.user.phone = Some(phone.into());
        self
    }
    
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.user.notes = Some(notes.into());
        self
    }
    
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.user.tags = Some(tags);
        self
    }
    
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.user.time_zone = Some(time_zone.into());
        self
    }
    
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.user.locale = Some(locale.into());
        self
    }
    
    pub fn build(self) -> UserCreateRequest {
        UserCreateRequest {
            user: self.user,
        }
    }
}