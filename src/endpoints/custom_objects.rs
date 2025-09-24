use crate::client::ZendeskClient;
use crate::errors::Result;
use crate::models::custom_object::{
    BulkJobRequest, BulkJobResponse, CreateCustomObjectFieldRequest,
    CreateCustomObjectRecordRequest, CreateCustomObjectRequest, CustomObject, CustomObjectField,
    CustomObjectFieldResponse, CustomObjectFieldsLimit, CustomObjectFieldsResponse,
    CustomObjectRecord, CustomObjectRecordCount, CustomObjectRecordResponse,
    CustomObjectRecordsResponse, CustomObjectResponse, CustomObjectsLimit, CustomObjectsResponse,
    JobStatus, ReorderCustomObjectFieldsRequest, SearchCustomObjectRecordsRequest,
    UpdateCustomObjectFieldRequest, UpdateCustomObjectRecordRequest, UpdateCustomObjectRequest,
    UpsertCustomObjectRecordRequest,
};

impl ZendeskClient {
    pub async fn create_custom_object(
        &self,
        request: CreateCustomObjectRequest,
    ) -> Result<CustomObject> {
        let response: CustomObjectResponse = self.post("custom_objects.json", &request).await?;
        Ok(response.custom_object)
    }

    pub async fn get_custom_object(&self, custom_object_key: &str) -> Result<CustomObject> {
        let endpoint = format!("custom_objects/{}.json", custom_object_key);
        let response: CustomObjectResponse = self.get(&endpoint).await?;
        Ok(response.custom_object)
    }

    pub async fn update_custom_object(
        &self,
        custom_object_key: &str,
        request: UpdateCustomObjectRequest,
    ) -> Result<CustomObject> {
        let endpoint = format!("custom_objects/{}.json", custom_object_key);
        let response: CustomObjectResponse = self.put(&endpoint, &request).await?;
        Ok(response.custom_object)
    }

    pub async fn delete_custom_object(&self, custom_object_key: &str) -> Result<()> {
        let endpoint = format!("custom_objects/{}.json", custom_object_key);
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    pub async fn list_custom_objects(&self) -> Result<Vec<CustomObject>> {
        let response: CustomObjectsResponse = self.get("custom_objects.json").await?;
        Ok(response.custom_objects)
    }

    pub async fn get_custom_objects_limit(&self) -> Result<CustomObjectsLimit> {
        let response: CustomObjectsLimit =
            self.get("custom_objects/limits/object_limit.json").await?;
        Ok(response)
    }

    // Custom Object Fields

    pub async fn list_custom_object_fields(
        &self,
        custom_object_key: &str,
        include_standard_fields: Option<bool>,
    ) -> Result<Vec<CustomObjectField>> {
        let mut endpoint = format!("custom_objects/{}/fields.json", custom_object_key);
        if let Some(include_standard) = include_standard_fields {
            endpoint.push_str(&format!("?include_standard_fields={}", include_standard));
        }
        let response: CustomObjectFieldsResponse = self.get(&endpoint).await?;
        Ok(response.custom_object_fields)
    }

    pub async fn create_custom_object_field(
        &self,
        custom_object_key: &str,
        request: CreateCustomObjectFieldRequest,
    ) -> Result<CustomObjectField> {
        let endpoint = format!("custom_objects/{}/fields.json", custom_object_key);
        let response: CustomObjectFieldResponse = self.post(&endpoint, &request).await?;
        Ok(response.custom_object_field)
    }

    pub async fn get_custom_object_field(
        &self,
        custom_object_key: &str,
        field_key_or_id: &str,
    ) -> Result<CustomObjectField> {
        let endpoint = format!(
            "custom_objects/{}/fields/{}.json",
            custom_object_key, field_key_or_id
        );
        let response: CustomObjectFieldResponse = self.get(&endpoint).await?;
        Ok(response.custom_object_field)
    }

    pub async fn update_custom_object_field(
        &self,
        custom_object_key: &str,
        field_key_or_id: &str,
        request: UpdateCustomObjectFieldRequest,
    ) -> Result<CustomObjectField> {
        let endpoint = format!(
            "custom_objects/{}/fields/{}.json",
            custom_object_key, field_key_or_id
        );
        let response: CustomObjectFieldResponse = self.patch(&endpoint, &request).await?;
        Ok(response.custom_object_field)
    }

    pub async fn delete_custom_object_field(
        &self,
        custom_object_key: &str,
        field_key_or_id: &str,
    ) -> Result<()> {
        let endpoint = format!(
            "custom_objects/{}/fields/{}.json",
            custom_object_key, field_key_or_id
        );
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    pub async fn reorder_custom_object_fields(
        &self,
        custom_object_key: &str,
        request: ReorderCustomObjectFieldsRequest,
    ) -> Result<Vec<CustomObjectField>> {
        let endpoint = format!("custom_objects/{}/fields/reorder.json", custom_object_key);
        let response: CustomObjectFieldsResponse = self.put(&endpoint, &request).await?;
        Ok(response.custom_object_fields)
    }

    pub async fn get_custom_object_fields_limit(
        &self,
        custom_object_key: &str,
    ) -> Result<CustomObjectFieldsLimit> {
        let endpoint = format!(
            "custom_objects/{}/limits/field_limit.json",
            custom_object_key
        );
        let response: CustomObjectFieldsLimit = self.get(&endpoint).await?;
        Ok(response)
    }

    // Custom Object Records

    pub async fn list_custom_object_records(
        &self,
        custom_object_key: &str,
        external_ids: Option<&[String]>,
        ids: Option<&[String]>,
        page_size: Option<u32>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<CustomObjectRecordsResponse> {
        let mut endpoint = format!("custom_objects/{}/records.json", custom_object_key);
        let mut params = Vec::new();

        if let Some(ext_ids) = external_ids {
            params.push(format!("external_ids={}", ext_ids.join(",")));
        }
        if let Some(record_ids) = ids {
            params.push(format!("ids={}", record_ids.join(",")));
        }
        if let Some(size) = page_size {
            params.push(format!("page[size]={}", size));
        }
        if let Some(sort) = sort_by {
            params.push(format!("sort={}", sort));
        }
        if let Some(order) = sort_order {
            params.push(format!("order={}", order));
        }

        if !params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&params.join("&"));
        }

        let response: CustomObjectRecordsResponse = self.get(&endpoint).await?;
        Ok(response)
    }

    pub async fn get_custom_object_record(
        &self,
        custom_object_key: &str,
        record_id: &str,
    ) -> Result<CustomObjectRecord> {
        let endpoint = format!(
            "custom_objects/{}/records/{}.json",
            custom_object_key, record_id
        );
        let response: CustomObjectRecordResponse = self.get(&endpoint).await?;
        Ok(response.custom_object_record)
    }

    pub async fn create_custom_object_record(
        &self,
        custom_object_key: &str,
        request: CreateCustomObjectRecordRequest,
    ) -> Result<CustomObjectRecord> {
        let endpoint = format!("custom_objects/{}/records.json", custom_object_key);
        let response: CustomObjectRecordResponse = self.post(&endpoint, &request).await?;
        Ok(response.custom_object_record)
    }

    pub async fn update_custom_object_record(
        &self,
        custom_object_key: &str,
        record_id: &str,
        request: UpdateCustomObjectRecordRequest,
    ) -> Result<CustomObjectRecord> {
        let endpoint = format!(
            "custom_objects/{}/records/{}.json",
            custom_object_key, record_id
        );
        let response: CustomObjectRecordResponse = self.patch(&endpoint, &request).await?;
        Ok(response.custom_object_record)
    }

    pub async fn upsert_custom_object_record(
        &self,
        custom_object_key: &str,
        request: UpsertCustomObjectRecordRequest,
    ) -> Result<CustomObjectRecord> {
        let endpoint = format!("custom_objects/{}/records.json", custom_object_key);
        let response: CustomObjectRecordResponse = self.patch(&endpoint, &request).await?;
        Ok(response.custom_object_record)
    }

    pub async fn delete_custom_object_record(
        &self,
        custom_object_key: &str,
        record_id: &str,
    ) -> Result<()> {
        let endpoint = format!(
            "custom_objects/{}/records/{}.json",
            custom_object_key, record_id
        );
        let _: serde_json::Value = self.delete(&endpoint).await?;
        Ok(())
    }

    pub async fn count_custom_object_records(
        &self,
        custom_object_key: &str,
    ) -> Result<CustomObjectRecordCount> {
        let endpoint = format!("custom_objects/{}/records/count.json", custom_object_key);
        let response: CustomObjectRecordCount = self.get(&endpoint).await?;
        Ok(response)
    }

    pub async fn search_custom_object_records_get(
        &self,
        custom_object_key: &str,
        query: Option<&str>,
        external_id: Option<&str>,
        name: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<CustomObjectRecordsResponse> {
        let mut endpoint = format!("custom_objects/{}/records/search.json", custom_object_key);
        let mut params = Vec::new();

        if let Some(q) = query {
            params.push(format!("query={}", urlencoding::encode(q)));
        }
        if let Some(ext_id) = external_id {
            params.push(format!("external_id={}", urlencoding::encode(ext_id)));
        }
        if let Some(record_name) = name {
            params.push(format!("name={}", urlencoding::encode(record_name)));
        }
        if let Some(size) = page_size {
            params.push(format!("page[size]={}", size));
        }

        if !params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&params.join("&"));
        }

        let response: CustomObjectRecordsResponse = self.get(&endpoint).await?;
        Ok(response)
    }

    pub async fn search_custom_object_records_post(
        &self,
        custom_object_key: &str,
        request: SearchCustomObjectRecordsRequest,
    ) -> Result<CustomObjectRecordsResponse> {
        let endpoint = format!("custom_objects/{}/records/search.json", custom_object_key);
        let response: CustomObjectRecordsResponse = self.post(&endpoint, &request).await?;
        Ok(response)
    }

    pub async fn create_bulk_job(
        &self,
        custom_object_key: &str,
        request: BulkJobRequest,
    ) -> Result<JobStatus> {
        let endpoint = format!("custom_objects/{}/jobs.json", custom_object_key);
        let response: BulkJobResponse = self.post(&endpoint, &request).await?;
        Ok(response.job_status)
    }

    pub async fn get_job_status(&self, custom_object_key: &str, job_id: &str) -> Result<JobStatus> {
        let endpoint = format!("custom_objects/{}/jobs/{}.json", custom_object_key, job_id);
        let response: BulkJobResponse = self.get(&endpoint).await?;
        Ok(response.job_status)
    }

    pub async fn incremental_export_custom_object_records(
        &self,
        custom_object_key: &str,
        cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<CustomObjectRecordsResponse> {
        let mut endpoint = format!(
            "incremental/custom_objects/{}/cursor.json",
            custom_object_key
        );
        let mut params = Vec::new();

        if let Some(cursor_val) = cursor {
            params.push(format!("cursor={}", cursor_val));
        }
        if let Some(size) = page_size {
            params.push(format!("page[size]={}", size));
        }

        if !params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&params.join("&"));
        }

        let response: CustomObjectRecordsResponse = self.get(&endpoint).await?;
        Ok(response)
    }
}
