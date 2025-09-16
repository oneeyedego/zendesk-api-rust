use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::user::{
    User, UserCreateRequest, UserResponse, UsersResponse
};

impl ZendeskClient {
    pub async fn create_user(&self, user_request: UserCreateRequest) -> Result<User> {
        let response: UserResponse = self.post("users.json", &user_request).await?;
        Ok(response.user)
    }
    
    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        let endpoint = format!("users/{}.json", user_id);
        let response: UserResponse = self.get(&endpoint).await?;
        Ok(response.user)
    }
    
    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let endpoint = format!("users/search.json?query=email:{}", email);
        let response: UsersResponse = self.get(&endpoint).await?;
        response.users.into_iter().next()
            .ok_or_else(|| crate::errors::ZendeskError::validation("User not found"))
    }
    
    pub async fn update_user(&self, user_id: u64, user_request: UserCreateRequest) -> Result<User> {
        let endpoint = format!("users/{}.json", user_id);
        let response: UserResponse = self.put(&endpoint, &user_request).await?;
        Ok(response.user)
    }
    
    pub async fn delete_user(&self, user_id: u64) -> Result<()> {
        let endpoint = format!("users/{}.json", user_id);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }
    
    pub async fn list_users(&self) -> Result<Vec<User>> {
        let response: UsersResponse = self.get("users.json").await?;
        Ok(response.users)
    }
    
    pub async fn list_users_in_organization(&self, organization_id: u64) -> Result<Vec<User>> {
        let endpoint = format!("organizations/{}/users.json", organization_id);
        let response: UsersResponse = self.get(&endpoint).await?;
        Ok(response.users)
    }
    
    pub async fn search_users(&self, query: &str) -> Result<Vec<User>> {
        let endpoint = format!("users/search.json?query={}", query);
        let response: UsersResponse = self.get(&endpoint).await?;
        Ok(response.users)
    }
}