use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    pub key: String,
    pub title: String,
    pub title_pluralized: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_list_view: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectResponse {
    pub custom_object: CustomObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectsResponse {
    pub custom_objects: Vec<CustomObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObjectRequest {
    pub custom_object: CreateCustomObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObject {
    pub key: String,
    pub title: String,
    pub title_pluralized: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_list_view: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObjectRequest {
    pub custom_object: UpdateCustomObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_pluralized: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_list_view: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectsLimit {
    pub object_limit: ObjectLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLimit {
    pub count: u64,
    pub limit: u64,
}

impl CreateCustomObject {
    pub fn new(key: String, title: String, title_pluralized: String) -> Self {
        Self {
            key,
            title,
            title_pluralized,
            description: None,
            allows_photos: None,
            include_in_list_view: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_allows_photos(mut self, allows_photos: bool) -> Self {
        self.allows_photos = Some(allows_photos);
        self
    }

    pub fn with_include_in_list_view(mut self, include_in_list_view: bool) -> Self {
        self.include_in_list_view = Some(include_in_list_view);
        self
    }
}

impl UpdateCustomObject {
    pub fn new() -> Self {
        Self {
            title: None,
            title_pluralized: None,
            description: None,
            allows_photos: None,
            include_in_list_view: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_title_pluralized(mut self, title_pluralized: String) -> Self {
        self.title_pluralized = Some(title_pluralized);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_allows_photos(mut self, allows_photos: bool) -> Self {
        self.allows_photos = Some(allows_photos);
        self
    }

    pub fn with_include_in_list_view(mut self, include_in_list_view: bool) -> Self {
        self.include_in_list_view = Some(include_in_list_view);
        self
    }
}

impl Default for UpdateCustomObject {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    pub key: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "type")]
    pub field_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp_for_validation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_target_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_filter: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_options: Option<Vec<CustomFieldOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    pub name: String,
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectFieldResponse {
    pub custom_object_field: CustomObjectField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectFieldsResponse {
    pub custom_object_fields: Vec<CustomObjectField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObjectFieldRequest {
    pub custom_object_field: CreateCustomObjectField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObjectField {
    pub key: String,
    pub title: String,

    #[serde(rename = "type")]
    pub field_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp_for_validation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_target_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_filter: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_options: Option<Vec<CreateCustomFieldOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldOption {
    pub name: String,
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObjectFieldRequest {
    pub custom_object_field: UpdateCustomObjectField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObjectField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp_for_validation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_target_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_filter: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_options: Option<Vec<CreateCustomFieldOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderCustomObjectFieldsRequest {
    pub custom_object_field_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectFieldsLimit {
    pub field_limit: FieldLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLimit {
    pub count: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_fields: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectRecordResponse {
    pub custom_object_record: CustomObjectRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectRecordsResponse {
    pub custom_object_records: Vec<CustomObjectRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObjectRecordRequest {
    pub custom_object_record: CreateCustomObjectRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomObjectRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObjectRecordRequest {
    pub custom_object_record: UpdateCustomObjectRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomObjectRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCustomObjectRecordRequest {
    pub custom_object_record: UpsertCustomObjectRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCustomObjectRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_fields: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomObjectRecordCount {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCustomObjectRecordsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkJobRequest {
    pub job: BulkJob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkJob {
    pub action: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkJobResponse {
    pub job_status: JobStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
}
