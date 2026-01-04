// Tower middleware for authentication, authorization, timeout, CORS, and tracing

use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info, warn};

use crate::core::auth::{ApiKey, ApiKeyId, AuthLevel, AuthResult};

/// API key store for authentication
/// In production, this would be backed by a database or external service
#[derive(Debug, Clone)]
pub struct ApiKeyStore {
    /// Map of API key to (key_id, auth_level)
    keys: Arc<RwLock<HashMap<String, (ApiKeyId, AuthLevel)>>>,
}

impl ApiKeyStore {
    /// Create a new API key store
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add an API key to the store
    pub async fn add_key(&self, key: String, key_id: ApiKeyId, auth_level: AuthLevel) {
        let mut keys = self.keys.write().await;
        keys.insert(key, (key_id, auth_level));
    }

    /// Validate an API key and return authentication result
    pub async fn validate_key(&self, key: &str) -> AuthResult {
        // First validate format
        let api_key = ApiKey::new(key.to_string());
        if let Err(reason) = api_key.validate_format() {
            return AuthResult::Unauthenticated { reason };
        }

        // Check if key exists in store
        let keys = self.keys.read().await;
        match keys.get(key) {
            Some((key_id, _)) => AuthResult::Authenticated {
                key_id: key_id.clone(),
            },
            None => AuthResult::Unauthenticated {
                reason: "API key not found".to_string(),
            },
        }
    }

    /// Get the authorization level for an API key
    pub async fn get_auth_level(&self, key: &str) -> Option<AuthLevel> {
        let keys = self.keys.read().await;
        keys.get(key).map(|(_, level)| *level)
    }

    /// Load API keys from environment variables
    /// Expects format: SENTINEL_API_KEY_<ID>=<KEY>:<LEVEL>
    /// Example: SENTINEL_API_KEY_VENDOR1=sk-1234567890123456:write
    pub async fn load_from_env(&self) -> Result<usize, String> {
        let mut count = 0;
        let mut keys = self.keys.write().await;

        for (key, value) in std::env::vars() {
            if key.starts_with("SENTINEL_API_KEY_") {
                let key_id_str = key.strip_prefix("SENTINEL_API_KEY_").unwrap();
                let key_id = ApiKeyId::new(key_id_str.to_string());

                // Parse value: <key>:<level>
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() != 2 {
                    warn!("Invalid API key format for {}: expected <key>:<level>", key);
                    continue;
                }

                let api_key = parts[0].to_string();
                let level_str = parts[1].to_lowercase();

                let auth_level = match level_str.as_str() {
                    "read" => AuthLevel::Read,
                    "write" => AuthLevel::Write,
                    "admin" => AuthLevel::Admin,
                    _ => {
                        warn!("Invalid auth level for {}: {}", key, level_str);
                        continue;
                    }
                };

                // Validate API key format
                let key_obj = ApiKey::new(api_key.clone());
                if key_obj.validate_format().is_err() {
                    warn!("Invalid API key format for {}", key);
                    continue;
                }

                keys.insert(api_key, (key_id, auth_level));
                count += 1;
                info!("Loaded API key: {}", key_id_str);
            }
        }

        Ok(count)
    }
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Request extension containing authentication information
#[derive(Debug, Clone)]
pub struct AuthInfo {
    /// API key ID of the authenticated key
    pub key_id: ApiKeyId,
    /// Authorization level
    pub auth_level: AuthLevel,
}

/// Extract API key from Authorization header
/// Supports both "Bearer <key>" and "ApiKey <key>" formats
fn extract_api_key(request: &Request) -> Option<String> {
    let auth_header = request.headers().get(AUTHORIZATION)?;
    let auth_str = auth_header.to_str().ok()?;

    // Try "Bearer <key>" format first (OpenAI-compatible)
    if let Some(key) = auth_str.strip_prefix("Bearer ") {
        return Some(key.trim().to_string());
    }

    // Try "ApiKey <key>" format
    if let Some(key) = auth_str.strip_prefix("ApiKey ") {
        return Some(key.trim().to_string());
    }

    // Try bare key (for compatibility)
    if !auth_str.contains(' ') {
        return Some(auth_str.to_string());
    }

    None
}

/// Authentication middleware
/// Validates API keys from Authorization header
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
    key_store: Arc<ApiKeyStore>,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    // Extract API key from header
    let api_key = match extract_api_key(&request) {
        Some(key) => key,
        None => {
            error!("Missing Authorization header");
            return Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": {
                        "code": "missing_authorization",
                        "message": "Authorization header is required",
                        "type": "authentication_error"
                    }
                })),
            ));
        }
    };

    // Validate API key
    let auth_result = key_store.validate_key(&api_key).await;
    match auth_result {
        AuthResult::Authenticated { key_id } => {
            // Get auth level
            let auth_level = key_store
                .get_auth_level(&api_key)
                .await
                .unwrap_or(AuthLevel::Read);

            let key_id_for_log = key_id.clone();

            // Add auth info to request extensions
            request
                .extensions_mut()
                .insert(AuthInfo { key_id, auth_level });

            info!("Authenticated request with key_id: {}", key_id_for_log);
            Ok(next.run(request).await)
        }
        AuthResult::Unauthenticated { reason } => {
            error!("Authentication failed: {}", reason);
            Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": {
                        "code": "invalid_api_key",
                        "message": format!("Authentication failed: {}", reason),
                        "type": "authentication_error"
                    }
                })),
            ))
        }
    }
}

/// Create authentication middleware with required authorization level
pub fn create_auth_middleware(
    key_store: Arc<ApiKeyStore>,
    required_level: AuthLevel,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, (StatusCode, axum::Json<serde_json::Value>)>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let store = key_store.clone();
        let level = required_level;
        Box::pin(async move {
            auth_with_level_middleware(request, next, store, level).await
        })
    }
}

/// Combined authentication and authorization middleware
/// Validates API key and checks if it has the required permission level
async fn auth_with_level_middleware(
    mut request: Request,
    next: Next,
    key_store: Arc<ApiKeyStore>,
    required_level: AuthLevel,
) -> Result<Response, (StatusCode, axum::Json<serde_json::Value>)> {
    // First authenticate
    let api_key = match extract_api_key(&request) {
        Some(key) => key,
        None => {
            error!("Missing Authorization header");
            return Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": {
                        "code": "missing_authorization",
                        "message": "Authorization header is required",
                        "type": "authentication_error"
                    }
                })),
            ));
        }
    };

    // Validate API key
    let auth_result = key_store.validate_key(&api_key).await;
    let (key_id, auth_level) = match auth_result {
        AuthResult::Authenticated { key_id } => {
            let level = key_store
                .get_auth_level(&api_key)
                .await
                .unwrap_or(AuthLevel::Read);
            (key_id, level)
        }
        AuthResult::Unauthenticated { reason } => {
            error!("Authentication failed: {}", reason);
            return Err((
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": {
                        "code": "invalid_api_key",
                        "message": format!("Authentication failed: {}", reason),
                        "type": "authentication_error"
                    }
                })),
            ));
        }
    };

    // Check authorization
    let has_permission = match required_level {
        AuthLevel::Read => auth_level.can_read(),
        AuthLevel::Write => auth_level.can_write(),
        AuthLevel::Admin => auth_level.is_admin(),
    };

    if !has_permission {
        error!(
            "Authorization failed: required {:?}, have {:?}",
            required_level, auth_level
        );
        return Err((
            StatusCode::FORBIDDEN,
            axum::Json(serde_json::json!({
                "error": {
                    "code": "insufficient_permissions",
                    "message": format!("Required {:?} access, but have {:?}", required_level, auth_level),
                    "type": "authorization_error"
                }
            })),
        ));
    }

    // Add auth info to request extensions
    request.extensions_mut().insert(AuthInfo {
        key_id: key_id.clone(),
        auth_level,
    });

    info!("Authenticated and authorized request with key_id: {}", key_id);
    Ok(next.run(request).await)
}

/// Create middleware stack with CORS and tracing
pub fn create_middleware_stack(
) -> impl tower::Layer<axum::routing::IntoMakeService<axum::Router>> + Clone {
    ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[tokio::test]
    async fn test_api_key_store_add_and_validate() {
        let store = ApiKeyStore::new();
        let key_id = ApiKeyId::new("test-key".to_string());
        let key = "sk-1234567890123456".to_string();

        store
            .add_key(key.clone(), key_id.clone(), AuthLevel::Write)
            .await;

        let result = store.validate_key(&key).await;
        match result {
            AuthResult::Authenticated { key_id: id } => {
                assert_eq!(id, key_id);
            }
            _ => panic!("Expected Authenticated"),
        }
    }

    #[tokio::test]
    async fn test_api_key_store_invalid_key() {
        let store = ApiKeyStore::new();
        let result = store.validate_key("invalid-key").await;

        match result {
            AuthResult::Unauthenticated { reason } => {
                assert!(reason.contains("not found") || reason.contains("at least 16"));
            }
            _ => panic!("Expected Unauthenticated"),
        }
    }

    #[tokio::test]
    async fn test_extract_api_key_bearer() {
        let mut request = Request::builder()
            .uri("http://example.com")
            .body(axum::body::Body::empty())
            .unwrap();

        request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str("Bearer sk-1234567890123456").unwrap(),
        );

        let key = extract_api_key(&request);
        assert_eq!(key, Some("sk-1234567890123456".to_string()));
    }

    #[tokio::test]
    async fn test_extract_api_key_apikey() {
        let mut request = Request::builder()
            .uri("http://example.com")
            .body(axum::body::Body::empty())
            .unwrap();

        request.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str("ApiKey sk-1234567890123456").unwrap(),
        );

        let key = extract_api_key(&request);
        assert_eq!(key, Some("sk-1234567890123456".to_string()));
    }

    #[tokio::test]
    async fn test_extract_api_key_missing() {
        let request = Request::builder()
            .uri("http://example.com")
            .body(axum::body::Body::empty())
            .unwrap();

        let key = extract_api_key(&request);
        assert_eq!(key, None);
    }

    #[tokio::test]
    async fn test_api_key_store_get_auth_level() {
        let store = ApiKeyStore::new();
        let key_id = ApiKeyId::new("test-key".to_string());
        let key = "sk-1234567890123456".to_string();

        store.add_key(key.clone(), key_id, AuthLevel::Admin).await;

        let level = store.get_auth_level(&key).await;
        assert_eq!(level, Some(AuthLevel::Admin));
    }
}
