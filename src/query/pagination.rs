use serde::{Deserialize, Serialize};

/// Cursor-based pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPagination {
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
}

/// Offset-based pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetPagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Combined pagination metadata that can handle both cursor and offset pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    // Cursor pagination fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,

    // Offset pagination fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

impl PaginationMeta {
    /// Check if this uses cursor-based pagination
    pub fn is_cursor_based(&self) -> bool {
        self.has_more.is_some() || self.after_cursor.is_some() || self.before_cursor.is_some()
    }

    /// Check if this uses offset-based pagination
    pub fn is_offset_based(&self) -> bool {
        self.next_page.is_some() || self.previous_page.is_some() || self.count.is_some()
    }

    /// Check if there are more pages available
    pub fn has_next_page(&self) -> bool {
        self.has_more.unwrap_or(false) || self.next_page.is_some()
    }

    /// Check if there are previous pages available
    pub fn has_previous_page(&self) -> bool {
        self.before_cursor.is_some() || self.previous_page.is_some()
    }

    /// Get the cursor for the next page (if using cursor pagination)
    pub fn next_cursor(&self) -> Option<&String> {
        self.after_cursor.as_ref()
    }

    /// Get the cursor for the previous page (if using cursor pagination)
    pub fn previous_cursor(&self) -> Option<&String> {
        self.before_cursor.as_ref()
    }

    /// Get the URL for the next page (if using offset pagination)
    pub fn next_page_url(&self) -> Option<&String> {
        self.next_page.as_ref()
    }

    /// Get the URL for the previous page (if using offset pagination)
    pub fn previous_page_url(&self) -> Option<&String> {
        self.previous_page.as_ref()
    }
}

/// Generic paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub results: Vec<T>,
    #[serde(flatten)]
    pub meta: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(results: Vec<T>) -> Self {
        Self {
            results,
            meta: PaginationMeta {
                has_more: None,
                before_cursor: None,
                after_cursor: None,
                next_page: None,
                previous_page: None,
                count: None,
                page: None,
                per_page: None,
            },
        }
    }

    pub fn with_cursor_pagination(
        results: Vec<T>,
        has_more: bool,
        after_cursor: Option<String>,
        before_cursor: Option<String>,
    ) -> Self {
        Self {
            results,
            meta: PaginationMeta {
                has_more: Some(has_more),
                after_cursor,
                before_cursor,
                next_page: None,
                previous_page: None,
                count: None,
                page: None,
                per_page: None,
            },
        }
    }

    pub fn with_offset_pagination(
        results: Vec<T>,
        count: Option<u64>,
        next_page: Option<String>,
        previous_page: Option<String>,
        page: Option<u32>,
        per_page: Option<u32>,
    ) -> Self {
        Self {
            results,
            meta: PaginationMeta {
                has_more: None,
                before_cursor: None,
                after_cursor: None,
                next_page,
                previous_page,
                count,
                page,
                per_page,
            },
        }
    }

    /// Check if there are more results available
    pub fn has_more_results(&self) -> bool {
        self.meta.has_next_page()
    }

    /// Get the total count of results (if available)
    pub fn total_count(&self) -> Option<u64> {
        self.meta.count
    }

    /// Get the number of results in this page
    pub fn page_size(&self) -> usize {
        self.results.len()
    }

    /// Check if this page is empty
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Map the results to a different type while preserving pagination metadata
    pub fn map<U, F>(self, f: F) -> PaginatedResponse<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResponse {
            results: self.results.into_iter().map(f).collect(),
            meta: self.meta,
        }
    }

    /// Filter results while preserving pagination metadata
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: FnMut(&T) -> bool,
    {
        self.results.retain(predicate);
        self
    }
}

/// Parameters for requesting paginated data
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    // Cursor pagination
    pub after_cursor: Option<String>,
    pub before_cursor: Option<String>,
    pub limit: Option<u32>,

    // Offset pagination
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub offset: Option<u32>,
}

impl PaginationParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set cursor for forward pagination
    pub fn after(mut self, cursor: String) -> Self {
        self.after_cursor = Some(cursor);
        self
    }

    /// Set cursor for backward pagination
    pub fn before(mut self, cursor: String) -> Self {
        self.before_cursor = Some(cursor);
        self
    }

    /// Set the limit for cursor-based pagination
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set page number for offset pagination
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set page size for offset pagination
    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Set offset for offset pagination
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Convert to query parameters string
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref cursor) = self.after_cursor {
            params.push(("page[after]".to_string(), cursor.clone()));
        }

        if let Some(ref cursor) = self.before_cursor {
            params.push(("page[before]".to_string(), cursor.clone()));
        }

        if let Some(limit) = self.limit {
            params.push(("page[size]".to_string(), limit.to_string()));
        }

        if let Some(page) = self.page {
            params.push(("page".to_string(), page.to_string()));
        }

        if let Some(per_page) = self.per_page {
            params.push(("per_page".to_string(), per_page.to_string()));
        }

        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_meta_cursor_based() {
        let meta = PaginationMeta {
            has_more: Some(true),
            after_cursor: Some("cursor123".to_string()),
            before_cursor: None,
            next_page: None,
            previous_page: None,
            count: None,
            page: None,
            per_page: None,
        };

        assert!(meta.is_cursor_based());
        assert!(!meta.is_offset_based());
        assert!(meta.has_next_page());
        assert!(!meta.has_previous_page());
        assert_eq!(meta.next_cursor(), Some(&"cursor123".to_string()));
    }

    #[test]
    fn test_pagination_meta_offset_based() {
        let meta = PaginationMeta {
            has_more: None,
            after_cursor: None,
            before_cursor: None,
            next_page: Some("https://example.com/api/v2/tickets.json?page=2".to_string()),
            previous_page: None,
            count: Some(100),
            page: Some(1),
            per_page: Some(25),
        };

        assert!(!meta.is_cursor_based());
        assert!(meta.is_offset_based());
        assert!(meta.has_next_page());
        assert!(!meta.has_previous_page());
        assert_eq!(meta.count, Some(100));
    }

    #[test]
    fn test_paginated_response_creation() {
        let data = vec!["item1".to_string(), "item2".to_string()];
        let response = PaginatedResponse::with_cursor_pagination(
            data,
            true,
            Some("next_cursor".to_string()),
            None,
        );

        assert_eq!(response.results.len(), 2);
        assert!(response.has_more_results());
        assert_eq!(response.page_size(), 2);
        assert!(!response.is_empty());
    }

    #[test]
    fn test_pagination_params_to_query_params() {
        let params = PaginationParams::new()
            .after("cursor123".to_string())
            .limit(50);

        let query_params = params.to_query_params();

        assert!(query_params.contains(&("page[after]".to_string(), "cursor123".to_string())));
        assert!(query_params.contains(&("page[size]".to_string(), "50".to_string())));
    }

    #[test]
    fn test_paginated_response_map() {
        let data = vec![1, 2, 3];
        let response = PaginatedResponse::new(data);

        let mapped = response.map(|x| x.to_string());

        assert_eq!(
            mapped.results,
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
    }

    #[test]
    fn test_paginated_response_filter() {
        let data = vec![1, 2, 3, 4, 5];
        let response = PaginatedResponse::new(data);

        let filtered = response.filter(|&x| x % 2 == 0);

        assert_eq!(filtered.results, vec![2, 4]);
        assert_eq!(filtered.page_size(), 2);
    }
}
