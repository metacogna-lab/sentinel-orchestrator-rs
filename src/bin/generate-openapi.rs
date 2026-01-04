// Binary to generate openapi.yaml from the OpenAPI schema
// Run with: cargo run --bin generate-openapi

use sentinel::api::routes::ApiDoc;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    
    // Convert to JSON first, then to YAML using serde_yaml
    let json = serde_json::to_string_pretty(&openapi)
        .expect("Failed to serialize OpenAPI spec to JSON");
    
    // Parse JSON and convert to YAML
    let value: serde_json::Value = serde_json::from_str(&json)
        .expect("Failed to parse OpenAPI JSON");
    let yaml = serde_yaml::to_string(&value)
        .expect("Failed to serialize OpenAPI spec to YAML");
    
    fs::write("openapi.yaml", yaml).expect("Failed to write openapi.yaml");
    println!("Generated openapi.yaml successfully");
}

