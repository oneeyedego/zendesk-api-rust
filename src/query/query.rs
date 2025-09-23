use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    pub include: Option<Vec<String>>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl QueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_include(mut self, include: Vec<String>) -> Self {
        self.include = Some(include);
        self
    }

    pub fn with_sideloading(mut self, resources: &[&str]) -> Self {
        self.include = Some(resources.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_per_page(mut self, per_page: u32) -> Self {
        self.per_page = Some(per_page);
        self
    }

    pub fn with_sort(mut self, sort_by: String, sort_order: SortOrder) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_order = Some(sort_order);
        self
    }

    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref include) = self.include {
            if !include.is_empty() {
                params.push(format!("include={}", include.join(",")));
            }
        }

        if let Some(page) = self.page {
            params.push(format!("page={}", page));
        }

        if let Some(per_page) = self.per_page {
            params.push(format!("per_page={}", per_page));
        }

        if let Some(ref sort_by) = self.sort_by {
            params.push(format!("sort_by={}", sort_by));
        }

        if let Some(ref sort_order) = self.sort_order {
            let order_str = match sort_order {
                SortOrder::Asc => "asc",
                SortOrder::Desc => "desc",
            };
            params.push(format!("sort_order={}", order_str));
        }

        if let Some(ref cursor) = self.cursor {
            params.push(format!("page[after]={}", cursor));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_query_params() {
        let params = QueryParams::new();
        assert_eq!(params.to_query_string(), "");
    }

    #[test]
    fn test_include_only() {
        let params =
            QueryParams::new().with_include(vec!["users".to_string(), "organizations".to_string()]);
        assert_eq!(params.to_query_string(), "?include=users,organizations");
    }

    #[test]
    fn test_sideloading_helper() {
        let params = QueryParams::new().with_sideloading(&["users", "organizations"]);
        assert_eq!(params.to_query_string(), "?include=users,organizations");
    }

    #[test]
    fn test_pagination() {
        let params = QueryParams::new().with_page(2).with_per_page(50);
        assert_eq!(params.to_query_string(), "?page=2&per_page=50");
    }

    #[test]
    fn test_sorting() {
        let params = QueryParams::new().with_sort("created_at".to_string(), SortOrder::Desc);
        assert_eq!(
            params.to_query_string(),
            "?sort_by=created_at&sort_order=desc"
        );
    }

    #[test]
    fn test_cursor_pagination() {
        let params = QueryParams::new()
            .with_cursor("eyJjcmVhdGVkX2F0IjoiMjAyMy0wMS0wMVQwMDowMDowMFoifQ==".to_string());
        assert_eq!(
            params.to_query_string(),
            "?page[after]=eyJjcmVhdGVkX2F0IjoiMjAyMy0wMS0wMVQwMDowMDowMFoifQ=="
        );
    }

    #[test]
    fn test_combined_params() {
        let params = QueryParams::new()
            .with_include(vec!["users".to_string()])
            .with_page(1)
            .with_per_page(25)
            .with_sort("updated_at".to_string(), SortOrder::Asc);

        let query_string = params.to_query_string();
        assert!(query_string.contains("include=users"));
        assert!(query_string.contains("page=1"));
        assert!(query_string.contains("per_page=25"));
        assert!(query_string.contains("sort_by=updated_at"));
        assert!(query_string.contains("sort_order=asc"));
    }
}
