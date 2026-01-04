// Build script to generate openapi.yaml from the OpenAPI schema
// Run with: cargo run --bin generate-openapi

use sentinel::api::routes::ApiDoc;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();
    let yaml = openapi.to_yaml().expect("Failed to serialize OpenAPI spec to YAML");
    
    fs::write("openapi.yaml", yaml).expect("Failed to write openapi.yaml");
    println!("Generated openapi.yaml successfully");
}

