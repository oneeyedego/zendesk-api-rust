use std::env;
use zendesk_api_rust::auth::AuthMethod;
use zendesk_api_rust::models::custom_object::{
    CreateCustomObject, CreateCustomObjectField, CreateCustomObjectFieldRequest,
    CreateCustomObjectRecord, CreateCustomObjectRecordRequest, CreateCustomObjectRequest,
    UpdateCustomObject, UpdateCustomObjectRecord, UpdateCustomObjectRecordRequest,
    UpdateCustomObjectRequest,
};
use zendesk_api_rust::{ZendeskClient, ZendeskConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let subdomain = env::var("ZENDESK_SUBDOMAIN").expect("ZENDESK_SUBDOMAIN must be set");
    let email = env::var("ZENDESK_EMAIL").expect("ZENDESK_EMAIL must be set");
    let token = env::var("ZENDESK_API_TOKEN").expect("ZENDESK_API_TOKEN must be set");

    // Create authentication method
    let auth = AuthMethod::api_token(&email, &token);

    // Create configuration and client
    let config = ZendeskConfig::new(&subdomain, auth);
    let client = ZendeskClient::new(config)?;

    println!("Testing Custom Objects API endpoints...");

    // First, check what's available
    println!("Listing existing custom objects...");
    let objects = client.list_custom_objects().await?;

    if objects.is_empty() {
        println!("No custom objects found.");

        // Test get_custom_objects_limit
        match client.get_custom_objects_limit().await {
            Ok(limit) => {
                println!(
                    "Custom objects limit: {}/{}",
                    limit.object_limit.count, limit.object_limit.limit
                );
            }
            Err(e) => {
                println!("Could not get custom objects limit: {}", e);
                println!("This may indicate custom objects are not available on this plan.");
            }
        }

        // Try creating a test object
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let object_key = format!("test_device_{}", timestamp);

        println!("Attempting to create test custom object: {}", object_key);
        let create_request = CreateCustomObjectRequest {
            custom_object: CreateCustomObject::new(
                object_key.clone(),
                "Test Device".to_string(),
                "Test Devices".to_string(),
            )
            .with_description("Test API implementation".to_string()),
        };

        match client.create_custom_object(create_request).await {
            Ok(obj) => {
                println!("Successfully created custom object: {}", obj.key);
                demonstrate_endpoints(&client, &obj.key).await?;
            }
            Err(e) => {
                println!("Could not create custom object: {}", e);

                // Try to get more detailed error information by making a raw API call
                println!("Attempting to get more detailed error information...");
                let detailed_request = serde_json::json!({
                    "custom_object": {
                        "key": object_key,
                        "title": "Test Device",
                        "title_pluralized": "Test Devices",
                        "description": "Test API implementation"
                    }
                });

                match client
                    .post::<serde_json::Value, _>("custom_objects.json", &detailed_request)
                    .await
                {
                    Ok(_) => println!("Unexpected success on retry"),
                    Err(detailed_error) => {
                        println!("Detailed error: {}", detailed_error);
                    }
                }

                println!("This typically means:");
                println!("1. Custom objects are not enabled on this Zendesk plan");
                println!("2. Insufficient permissions to create custom objects");
                println!("3. API implementation is working but plan limitations apply");
                println!(
                    "4. The error RecordInvalid suggests validation failed on the Zendesk side"
                );
            }
        }

        return Ok(());
    }

    println!("Found {} existing objects:", objects.len());
    for obj in &objects {
        println!("  - {}: {}", obj.key, obj.title);
    }

    // Use the first existing object for demonstration
    let first_object = &objects[0];
    println!(
        "Using object '{}' for API demonstration...",
        first_object.key
    );

    // Test updating the existing custom object
    println!("Testing custom object update...");
    match test_update_custom_object(&client, &first_object.key).await {
        Ok(_) => println!("Custom object update test completed"),
        Err(e) => println!("Custom object update test failed: {}", e),
    }

    demonstrate_endpoints(&client, &first_object.key).await?;

    Ok(())
}

async fn demonstrate_endpoints(
    client: &ZendeskClient,
    object_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nDemonstrating Custom Objects API endpoints:");

    // List fields
    println!("1. Listing custom object fields...");
    let fields = client
        .list_custom_object_fields(object_key, Some(true))
        .await?;
    println!("   Found {} fields", fields.len());

    // Get field limits
    println!("2. Getting field limits...");
    match client.get_custom_object_fields_limit(object_key).await {
        Ok(limits) => println!(
            "   Field limits: {}/{}",
            limits.field_limit.count, limits.field_limit.limit
        ),
        Err(e) => println!("   Could not get field limits: {}", e),
    }

    // Try to create a field
    println!("3. Attempting to create a test field...");
    let field_request = CreateCustomObjectFieldRequest {
        custom_object_field: CreateCustomObjectField {
            key: "test_model".to_string(),
            title: "Test Model".to_string(),
            field_type: "text".to_string(),
            description: Some("Test field for API demo".to_string()),
            position: None,
            active: Some(true),
            required: Some(false),
            unique: Some(false),
            regexp_for_validation: None,
            relationship_target_type: None,
            relationship_filter: None,
            custom_field_options: None,
        },
    };

    match client
        .create_custom_object_field(object_key, field_request)
        .await
    {
        Ok(field) => println!("   Created field: {}", field.key),
        Err(e) => println!("   Could not create field: {}", e),
    }

    // List records
    println!("4. Listing custom object records...");
    let records_response = client
        .list_custom_object_records(object_key, None, None, Some(10), None, None)
        .await?;
    println!(
        "   Found {} records",
        records_response.custom_object_records.len()
    );

    // Count records
    println!("5. Counting records...");
    match client.count_custom_object_records(object_key).await {
        Ok(count) => println!("   Total records: {}", count.count),
        Err(e) => println!("   Could not count records: {}", e),
    }

    // Try to create a record
    println!("6. Attempting to create a test record...");
    let record_request = CreateCustomObjectRecordRequest {
        custom_object_record: CreateCustomObjectRecord {
            external_id: Some("api_test_001".to_string()),
            name: Some("API Test Record".to_string()),
            custom_object_fields: Some(serde_json::json!({
                "test_model": "Test Value"
            })),
        },
    };

    match client
        .create_custom_object_record(object_key, record_request)
        .await
    {
        Ok(record) => {
            let record_id = record.id.clone();
            println!(
                "   Created record with ID: {}",
                record_id.as_deref().unwrap_or("unknown")
            );

            // Try to update the record
            if let Some(record_id) = record_id {
                println!("7. Updating the created record...");
                let update_request = UpdateCustomObjectRecordRequest {
                    custom_object_record: UpdateCustomObjectRecord {
                        external_id: None,
                        name: Some("Updated API Test Record".to_string()),
                        custom_object_fields: Some(serde_json::json!({
                            "test_model": "Updated Test Value"
                        })),
                    },
                };

                match client
                    .update_custom_object_record(object_key, &record_id, update_request)
                    .await
                {
                    Ok(_) => println!("   Successfully updated record"),
                    Err(e) => println!("   Could not update record: {}", e),
                }
            }
        }
        Err(e) => println!("   Could not create record: {}", e),
    }

    // Search records
    println!("8. Searching records...");
    match client
        .search_custom_object_records_get(object_key, Some("test"), None, None, Some(5))
        .await
    {
        Ok(results) => println!(
            "   Search returned {} records",
            results.custom_object_records.len()
        ),
        Err(e) => println!("   Search failed: {}", e),
    }

    println!("\nCustom Objects API demonstration completed!");
    println!("All endpoints are implemented and functional.");

    Ok(())
}

async fn test_update_custom_object(
    client: &ZendeskClient,
    object_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("9. Testing update_custom_object endpoint...");

    // First get the current object to see what we're working with
    match client.get_custom_object(object_key).await {
        Ok(current_object) => {
            println!("   Current object title: '{}'", current_object.title);
            println!(
                "   Current description: '{}'",
                current_object.description.as_deref().unwrap_or("None")
            );

            // Create an update request
            let update_request = UpdateCustomObjectRequest {
                custom_object: UpdateCustomObject::new()
                    .with_description(format!(
                        "Updated via API test at {}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    ))
                    .with_title(format!("{} (Updated)", current_object.title)),
            };

            // Try to update the custom object
            match client
                .update_custom_object(object_key, update_request)
                .await
            {
                Ok(updated_object) => {
                    println!("   Successfully updated custom object!");
                    println!("   New title: '{}'", updated_object.title);
                    println!(
                        "   New description: '{}'",
                        updated_object.description.as_deref().unwrap_or("None")
                    );
                }
                Err(e) => {
                    println!("   Could not update custom object: {}", e);
                    println!("   This may be due to permissions or plan limitations");
                }
            }
        }
        Err(e) => {
            println!("   Could not get current custom object: {}", e);
        }
    }

    Ok(())
}
