// Removed pact_verifier dependency - using simplified approach
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;

/// Shared utilities for Pact verification testing in Rust
pub struct PactTestHelper {
    pub pact_dir: PathBuf,
    pub log_dir: PathBuf,
}

impl PactTestHelper {
    /// Create a new PactTestHelper with default directories
    pub fn new() -> Self {
        Self {
            pact_dir: PathBuf::from("../../pacts/consumer-contracts"),
            log_dir: PathBuf::from("../../pacts/provider-verification"),
        }
    }

    /// Set up provider verification configuration for HTTP-based contracts
    pub fn setup_provider_verification(&self, provider_name: &str, provider_url: &str) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("name".to_string(), provider_name.to_string());
        config.insert("host".to_string(), provider_url.to_string());
        config.insert("path".to_string(), "/".to_string());
        config.insert("protocol".to_string(), "http".to_string());
        config
    }

    /// Load pact files from the consumer contracts directory
    pub fn load_pact_files(&self, consumer_name: &str, provider_name: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let pact_file = self.pact_dir.join(format!("{}-{}.json", consumer_name.to_lowercase(), provider_name.to_lowercase()));
        
        if !pact_file.exists() {
            return Err(format!("Pact file not found: {:?}", pact_file).into());
        }

        Ok(vec![pact_file])
    }

    /// Validate that a pact file has the expected structure
    pub fn validate_pact_file(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let pact: Value = serde_json::from_str(&content)?;

        // Basic validation
        if !pact.get("consumer").is_some() {
            return Err("Pact file missing consumer".into());
        }
        
        if !pact.get("provider").is_some() {
            return Err("Pact file missing provider".into());
        }
        
        if !pact.get("interactions").and_then(|i| i.as_array()).is_some() {
            return Err("Pact file missing interactions array".into());
        }

        Ok(())
    }

    /// Set up provider state for testing different scenarios
    pub fn setup_provider_state(&self, state: &str) -> HashMap<String, Value> {
        match state {
            "valid shipping request" => {
                let mut state_data = HashMap::new();
                state_data.insert("products".to_string(), json!([
                    {"product_id": "OLJCESPC7Z", "name": "Vintage Typewriter"},
                    {"product_id": "66VCHSJNUP", "name": "Vintage Camera Lens"}
                ]));
                state_data
            },
            "empty cart request" => {
                let mut state_data = HashMap::new();
                state_data.insert("products".to_string(), json!([]));
                state_data
            },
            _ => HashMap::new()
        }
    }
}

impl Default for PactTestHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Test data factory for consistent test data across Rust tests
pub struct TestDataFactory;

impl TestDataFactory {
    /// Get valid shipping quote request data
    pub fn get_valid_shipping_request() -> Value {
        json!({
            "items": [
                {"product_id": "OLJCESPC7Z", "quantity": 2},
                {"product_id": "66VCHSJNUP", "quantity": 1}
            ],
            "address": {
                "street_address": "1600 Amphitheatre Parkway",
                "city": "Mountain View",
                "state": "CA",
                "country": "United States",
                "zip_code": "94043"
            }
        })
    }

    /// Get expected shipping quote response data
    pub fn get_valid_shipping_response() -> Value {
        json!({
            "cost_usd": {
                "currency_code": "USD",
                "units": 8,
                "nanos": 990000000
            }
        })
    }

    /// Get empty cart request for error testing
    pub fn get_empty_cart_request() -> Value {
        json!({
            "items": [],
            "address": {
                "street_address": "1600 Amphitheatre Parkway",
                "city": "Mountain View",
                "state": "CA",
                "country": "United States",
                "zip_code": "94043"
            }
        })
    }

    /// Get invalid address request for error testing
    pub fn get_invalid_address_request() -> Value {
        json!({
            "items": [
                {"product_id": "OLJCESPC7Z", "quantity": 1}
            ],
            "address": {
                "street_address": "",
                "city": "",
                "state": "",
                "country": "",
                "zip_code": ""
            }
        })
    }

    /// Get expected error response format
    pub fn get_error_response(status: u16, message: &str) -> Value {
        json!({
            "error": {
                "code": "VALIDATION_ERROR",
                "message": message,
                "status": status
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pact_helper_creation() {
        let helper = PactTestHelper::new();
        assert!(helper.pact_dir.to_string_lossy().contains("consumer-contracts"));
        assert!(helper.log_dir.to_string_lossy().contains("provider-verification"));
    }

    #[test]
    fn test_test_data_factory() {
        let request = TestDataFactory::get_valid_shipping_request();
        assert!(request.get("items").is_some());
        assert!(request.get("address").is_some());

        let response = TestDataFactory::get_valid_shipping_response();
        assert!(response.get("cost_usd").is_some());
    }
}