use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::search::{
    SearchCountResponse, SearchExportResponse, SearchQueryBuilder, SearchResponse, SearchSortBy,
};
use crate::query::SortOrder;

impl ZendeskClient {
    /// Search across tickets, users, organizations, and groups
    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        let endpoint = format!("search.json?query={}", urlencoding::encode(query));
        self.get(&endpoint).await
    }

    /// Search with sorting options
    pub async fn search_with_sort(
        &self,
        query: &str,
        sort_by: SearchSortBy,
        order: SortOrder,
    ) -> Result<SearchResponse> {
        let endpoint = format!(
            "search.json?query={}&sort_by={}&sort_order={}",
            urlencoding::encode(query),
            sort_by,
            order
        );
        self.get(&endpoint).await
    }

    /// Search with pagination using page URLs
    pub async fn search_with_pagination(&self, page_url: &str) -> Result<SearchResponse> {
        // Extract just the path and query from the full URL
        let endpoint = if let Some(path_start) = page_url.find("/api/v2/") {
            &page_url[path_start + 8..] // Skip "/api/v2/"
        } else {
            return Err(crate::errors::ZendeskError::InvalidUrl(
                page_url.to_string(),
            ));
        };
        self.get(endpoint).await
    }

    /// Count search results
    pub async fn search_count(&self, query: &str) -> Result<u64> {
        let endpoint = format!("search/count.json?query={}", urlencoding::encode(query));
        let response: SearchCountResponse = self.get(&endpoint).await?;
        Ok(response.count)
    }

    /// Export search results (for large result sets over 1,000 results)
    pub async fn search_export(&self, query: &str) -> Result<SearchExportResponse> {
        let endpoint = format!("search/export.json?query={}", urlencoding::encode(query));
        self.get(&endpoint).await
    }

    /// Export search results with cursor pagination
    pub async fn search_export_with_cursor(
        &self,
        query: &str,
        cursor: Option<&str>,
    ) -> Result<SearchExportResponse> {
        let mut endpoint = format!("search/export.json?query={}", urlencoding::encode(query));

        if let Some(cursor_value) = cursor {
            endpoint.push_str(&format!(
                "&page[after]={}",
                urlencoding::encode(cursor_value)
            ));
        }

        self.get(&endpoint).await
    }

    // Convenience methods for specific resource types

    /// Search only tickets (returns SearchResponse with full search metadata)
    pub async fn search_tickets_advanced(&self, query: &str) -> Result<SearchResponse> {
        let full_query = if query.contains("type:ticket") {
            query.to_string()
        } else {
            format!("type:ticket {}", query)
        };
        self.search(&full_query).await
    }

    /// Search only users (returns SearchResponse with full search metadata)
    pub async fn search_users_advanced(&self, query: &str) -> Result<SearchResponse> {
        let full_query = if query.contains("type:user") {
            query.to_string()
        } else {
            format!("type:user {}", query)
        };
        self.search(&full_query).await
    }

    /// Search only organizations (returns SearchResponse with full search metadata)
    pub async fn search_organizations_advanced(&self, query: &str) -> Result<SearchResponse> {
        let full_query = if query.contains("type:organization") {
            query.to_string()
        } else {
            format!("type:organization {}", query)
        };
        self.search(&full_query).await
    }

    /// Search only groups
    pub async fn search_groups(&self, query: &str) -> Result<SearchResponse> {
        let full_query = if query.contains("type:group") {
            query.to_string()
        } else {
            format!("type:group {}", query)
        };
        self.search(&full_query).await
    }

    /// Advanced search using query builder
    pub async fn search_advanced(&self, builder: SearchQueryBuilder) -> Result<SearchResponse> {
        let query = builder.build();
        self.search(&query).await
    }

    /// Advanced search with sorting using query builder
    pub async fn search_advanced_with_sort(
        &self,
        builder: SearchQueryBuilder,
        sort_by: SearchSortBy,
        order: SortOrder,
    ) -> Result<SearchResponse> {
        let query = builder.build();
        self.search_with_sort(&query, sort_by, order).await
    }
}
