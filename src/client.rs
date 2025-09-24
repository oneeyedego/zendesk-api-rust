use crate::config::ZendeskConfig;
use crate::errors::{Result, ZendeskError};
use crate::query::QueryParams;
use crate::query::SideloadedResponse;
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct ZendeskClient {
    config: ZendeskConfig,
    http_client: Client,
    base_url: Url,
}

impl ZendeskClient {
    pub fn new(config: ZendeskConfig) -> Result<Self> {
        config.validate()?;

        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(ZendeskError::from)?;

        let base_url = config.base_url()?;

        Ok(Self {
            config,
            http_client,
            base_url,
        })
    }

    pub fn config(&self) -> &ZendeskConfig {
        &self.config
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::GET, endpoint, Option::<&()>::None, None)
            .await?;
        self.handle_response(response).await
    }

    pub async fn get_with_params<T>(&self, endpoint: &str, params: &QueryParams) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::GET, endpoint, Option::<&()>::None, Some(params))
            .await?;
        self.handle_response(response).await
    }

    pub async fn get_with_sideloading<T>(
        &self,
        endpoint: &str,
        include: &[&str],
    ) -> Result<SideloadedResponse<T>>
    where
        T: DeserializeOwned,
    {
        let params = QueryParams::new().with_sideloading(include);
        let response = self
            .request(Method::GET, endpoint, Option::<&()>::None, Some(&params))
            .await?;
        self.handle_response(response).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::POST, endpoint, Some(body), None)
            .await?;
        self.handle_response(response).await
    }

    pub async fn post_with_params<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        params: &QueryParams,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::POST, endpoint, Some(body), Some(params))
            .await?;
        self.handle_response(response).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::PUT, endpoint, Some(body), None)
            .await?;
        self.handle_response(response).await
    }

    pub async fn put_with_params<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        params: &QueryParams,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::PUT, endpoint, Some(body), Some(params))
            .await?;
        self.handle_response(response).await
    }

    pub async fn patch<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::PATCH, endpoint, Some(body), None)
            .await?;
        self.handle_response(response).await
    }

    pub async fn patch_with_params<T, B>(
        &self,
        endpoint: &str,
        body: &B,
        params: &QueryParams,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self
            .request(Method::PATCH, endpoint, Some(body), Some(params))
            .await?;
        self.handle_response(response).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::DELETE, endpoint, Option::<&()>::None, None)
            .await?;
        self.handle_response(response).await
    }

    pub async fn delete_with_params<T>(&self, endpoint: &str, params: &QueryParams) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .request(Method::DELETE, endpoint, Option::<&()>::None, Some(params))
            .await?;
        self.handle_response(response).await
    }

    async fn request<B>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&B>,
        params: Option<&QueryParams>,
    ) -> Result<Response>
    where
        B: Serialize,
    {
        let url = self.build_url_with_params(endpoint, params)?;
        let mut request = self.http_client.request(method, url);

        request = self.add_headers(request)?;

        if let Some(body) = body {
            // Debug output for user creation requests
            if std::env::var("ZENDESK_DEBUG").is_ok() {
                if let Ok(json_str) = serde_json::to_string_pretty(body) {
                    println!("Debug - Request body: {}", json_str);
                }
            }
            request = request.json(body);
        }

        let response = request.send().await.map_err(ZendeskError::from)?;

        Ok(response)
    }

    fn build_url_with_params(&self, endpoint: &str, params: Option<&QueryParams>) -> Result<Url> {
        let endpoint = endpoint.trim_start_matches('/');
        let mut url = self.base_url.join(endpoint).map_err(ZendeskError::from)?;

        if let Some(params) = params {
            let query_string = params.to_query_string();
            if !query_string.is_empty() {
                // Remove the leading '?' if present
                let query = query_string.trim_start_matches('?');
                url.set_query(Some(query));
            }
        }

        Ok(url)
    }

    fn add_headers(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        let auth_header = self.config.auth.to_header_value();
        let mut request = request
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        if let Some(ref user_agent) = self.config.user_agent {
            request = request.header("User-Agent", user_agent);
        }

        Ok(request)
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();

        if status.is_success() {
            let json: T = response.json().await.map_err(ZendeskError::from)?;
            Ok(json)
        } else {
            let error_text = response.text().await.unwrap_or_default();

            // Try to parse error as JSON to get more details
            let error_message = if let Ok(error_json) = serde_json::from_str::<Value>(&error_text) {
                // For 422 errors, try to get detailed validation errors
                if status.as_u16() == 422 {
                    if let Some(details) = error_json.get("details") {
                        format!("RecordInvalid - Details: {}", details.to_string())
                    } else if let Some(errors) = error_json.get("errors") {
                        format!("RecordInvalid - Errors: {}", errors.to_string())
                    } else if let Some(description) = error_json.get("description") {
                        format!(
                            "RecordInvalid - {}",
                            description.as_str().unwrap_or("Unknown validation error")
                        )
                    } else {
                        error_json
                            .get("error")
                            .and_then(|e| e.as_str())
                            .unwrap_or(&error_text)
                            .to_string()
                    }
                } else {
                    error_json
                        .get("error")
                        .and_then(|e| e.as_str())
                        .unwrap_or(&error_text)
                        .to_string()
                }
            } else {
                error_text
            };

            match status.as_u16() {
                401 => Err(ZendeskError::auth(error_message)),
                429 => {
                    // TODO: Parse Retry-After header for rate limiting
                    Err(ZendeskError::RateLimit { retry_after: None })
                }
                _ => Err(ZendeskError::api(status.as_u16(), error_message)),
            }
        }
    }
}
