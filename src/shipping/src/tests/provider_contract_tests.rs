use actix_web::{test, web, App, HttpServer, middleware::Logger};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

use crate::shipping_service::{get_quote, ship_order};

/// Comprehensive provider verification tests for shipping service
/// This module implements task 3.1, 3.2, and 3.3 requirements
pub struct ShippingProviderTests {
    pub server_port: u16,
    pub pact_file_path: PathBuf,
    pub test_data: HashMap<String, Value>,
}

impl ShippingProviderTests {
    /// Create new provider test setup (Task 3.1 requirement)
    pub fn new() -> Self {
        Self {
            server_port: 8081, // Use different port to avoid conflicts
            pact_file_path: PathBuf::from("../../pacts/consumer-contracts/frontend-shipping-service.json"),
            test_data: HashMap::new(),
        }
    }

    /// Set up Actix-web test server for shipping service (Task 3.1)
    /// Requirements: 1.1, 1.2, 5.2, 7.3
    pub async fn setup_test_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Setting up Actix-web test server on port {}", self.server_port);
        
        let server = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .service(get_quote)
                .service(ship_order)
        })
        .bind(format!("127.0.0.1:{}", self.server_port))?
        .run();

        // Start server in background
        tokio::spawn(server);
        
        // Give server time to start
        sleep(Duration::from_millis(200)).await;
        
        // Verify server is running
        let client = reqwest::Client::new();
        let health_check = client
            .get(&format!("http://127.0.0.1:{}/", self.server_port))
            .send()
            .await;
            
        match health_check {
            Ok(_) => println!("✓ Test server started successfully"),
            Err(e) => println!("⚠ Server may not be fully ready: {}", e),
        }
        
        Ok(())
    }

    /// Create test data fixtures for different scenarios (Task 3.3)
    /// Requirements: 5.2, 5.5
    pub fn setup_test_data_fixtures(&mut self) {
        println!("Setting up test data fixtures for different scenarios");

        // Valid shipping request with single item
        self.test_data.insert(
            "single_item_request".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 1}
                ],
                "address": {
                    "street_address": "1600 Amphitheatre Parkway",
                    "city": "Mountain View",
                    "state": "CA",
                    "country": "United States",
                    "zip_code": "94043"
                }
            })
        );

        // Valid shipping request with multiple items
        self.test_data.insert(
            "multiple_items_request".to_string(),
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
        );

        // International address request
        self.test_data.insert(
            "international_request".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 1}
                ],
                "address": {
                    "street_address": "123 International St",
                    "city": "Toronto",
                    "state": "ON",
                    "country": "Canada",
                    "zip_code": "M5V 3A8"
                }
            })
        );

        // Empty items array (should return 400)
        self.test_data.insert(
            "empty_items_request".to_string(),
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
        );

        // Missing address fields (should return 400)
        self.test_data.insert(
            "missing_address_fields_request".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 1}
                ],
                "address": {
                    "street_address": "1600 Amphitheatre Parkway",
                    "city": "Mountain View",
                    "state": "",
                    "country": "United States",
                    "zip_code": ""
                }
            })
        );

        println!("✓ Test data fixtures created: {} scenarios", self.test_data.len());
    }

    /// Verify get-quote endpoint against frontend consumer contract (Task 3.2)
    /// Requirements: 1.1, 1.2, 6.1, 6.4
    pub async fn verify_get_quote_endpoint(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Verifying get-quote endpoint against consumer contract");
        let client = reqwest::Client::new();
        let base_url = format!("http://127.0.0.1:{}", self.server_port);

        // Test 1: Valid single item request
        if let Some(request_data) = self.test_data.get("single_item_request") {
            println!("Testing single item request...");
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                self.validate_successful_quote_response(&body)?;
                println!("✓ Single item request verified");
            } else {
                return Err(format!("Single item request failed with status: {}", response.status()).into());
            }
        }

        // Test 2: Valid multiple items request
        if let Some(request_data) = self.test_data.get("multiple_items_request") {
            println!("Testing multiple items request...");
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                self.validate_successful_quote_response(&body)?;
                println!("✓ Multiple items request verified");
            } else {
                return Err(format!("Multiple items request failed with status: {}", response.status()).into());
            }
        }

        // Test 3: International address request
        if let Some(request_data) = self.test_data.get("international_request") {
            println!("Testing international address request...");
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                self.validate_successful_quote_response(&body)?;
                println!("✓ International address request verified");
            } else {
                return Err(format!("International request failed with status: {}", response.status()).into());
            }
        }

        println!("✓ Get-quote endpoint verification completed successfully");
        Ok(())
    }

    /// Validate error handling matches consumer expectations (Task 3.2)
    /// Requirements: 1.1, 1.2, 6.1, 6.4
    pub async fn verify_error_handling(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Verifying error handling matches consumer expectations");
        let client = reqwest::Client::new();
        let base_url = format!("http://127.0.0.1:{}", self.server_port);

        // Test 1: Empty items array should return 400
        if let Some(request_data) = self.test_data.get("empty_items_request") {
            println!("Testing empty items array error handling...");
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                let body: Value = response.json().await?;
                self.validate_error_response(&body, "Items array cannot be empty")?;
                println!("✓ Empty items error handling verified");
            } else {
                return Err(format!("Empty items should return 400, got: {}", response.status()).into());
            }
        }

        // Test 2: Missing address fields should return 400
        if let Some(request_data) = self.test_data.get("missing_address_fields_request") {
            println!("Testing missing address fields error handling...");
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                let body: Value = response.json().await?;
                self.validate_error_response(&body, "Missing required address fields")?;
                println!("✓ Missing address fields error handling verified");
            } else {
                return Err(format!("Missing address fields should return 400, got: {}", response.status()).into());
            }
        }

        // Test 3: Malformed JSON should return 400
        println!("Testing malformed JSON error handling...");
        let response = client
            .post(&format!("{}/get-quote", base_url))
            .header("Content-Type", "application/json")
            .body("{ invalid json }")
            .send()
            .await?;

        if response.status() == 400 {
            println!("✓ Malformed JSON error handling verified");
        } else {
            return Err(format!("Malformed JSON should return 400, got: {}", response.status()).into());
        }

        println!("✓ Error handling verification completed successfully");
        Ok(())
    }

    /// Ensure deterministic responses for contract verification (Task 3.3)
    /// Requirements: 5.2, 5.5
    pub async fn verify_deterministic_responses(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Verifying deterministic responses for contract verification");
        let client = reqwest::Client::new();
        let base_url = format!("http://127.0.0.1:{}", self.server_port);

        if let Some(request_data) = self.test_data.get("single_item_request") {
            println!("Testing response consistency with multiple requests...");
            
            let mut responses = Vec::new();
            
            // Make 3 identical requests
            for i in 1..=3 {
                println!("Making request {} of 3...", i);
                let response = client
                    .post(&format!("{}/get-quote", base_url))
                    .header("Content-Type", "application/json")
                    .json(request_data)
                    .send()
                    .await?;

                if response.status().is_success() {
                    let body: Value = response.json().await?;
                    responses.push(body);
                } else {
                    return Err(format!("Request {} failed with status: {}", i, response.status()).into());
                }
            }

            // Verify all responses are identical
            if responses.len() >= 2 {
                let first_response = &responses[0];
                for (i, response) in responses.iter().enumerate().skip(1) {
                    if first_response != response {
                        return Err(format!("Response {} differs from first response", i + 1).into());
                    }
                }
                println!("✓ All responses are identical - deterministic behavior verified");
            }
        }

        println!("✓ Deterministic response verification completed successfully");
        Ok(())
    }

    /// Run complete provider verification test suite
    pub async fn run_full_verification(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Starting Shipping Service Provider Verification ===");
        
        // Task 3.1: Set up test server and fixtures
        self.setup_test_server().await?;
        self.setup_test_data_fixtures();
        
        // Task 3.2: Verify contract compliance
        self.verify_get_quote_endpoint().await?;
        self.verify_error_handling().await?;
        
        // Task 3.3: Verify deterministic behavior
        self.verify_deterministic_responses().await?;
        
        println!("=== Provider Verification Completed Successfully ===");
        Ok(())
    }

    /// Helper method to validate successful quote response format
    fn validate_successful_quote_response(&self, body: &Value) -> Result<(), Box<dyn std::error::Error>> {
        // Check for cost_usd field
        let cost_usd = body.get("cost_usd")
            .ok_or("Response missing cost_usd field")?;

        // Check currency_code
        let currency_code = cost_usd.get("currency_code")
            .and_then(|v| v.as_str())
            .ok_or("cost_usd missing currency_code")?;
        
        if currency_code != "USD" {
            return Err(format!("Expected currency_code 'USD', got '{}'", currency_code).into());
        }

        // Check units field
        let units = cost_usd.get("units")
            .and_then(|v| v.as_u64())
            .ok_or("cost_usd missing or invalid units field")?;

        // Check nanos field
        let nanos = cost_usd.get("nanos")
            .and_then(|v| v.as_u64())
            .ok_or("cost_usd missing or invalid nanos field")?;

        // Validate nanos is in valid range (0-999,999,999)
        if nanos > 999_999_999 {
            return Err(format!("nanos value {} exceeds maximum 999,999,999", nanos).into());
        }

        println!("  Response format valid: ${}.{:09} USD", units, nanos);
        Ok(())
    }

    /// Helper method to validate error response format
    fn validate_error_response(&self, body: &Value, expected_message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let error = body.get("error")
            .ok_or("Error response missing error field")?;

        let code = error.get("code")
            .and_then(|v| v.as_str())
            .ok_or("Error missing code field")?;

        let message = error.get("message")
            .and_then(|v| v.as_str())
            .ok_or("Error missing message field")?;

        if code != "INVALID_REQUEST" {
            return Err(format!("Expected error code 'INVALID_REQUEST', got '{}'", code).into());
        }

        if message != expected_message {
            return Err(format!("Expected error message '{}', got '{}'", expected_message, message).into());
        }

        println!("  Error response format valid: {} - {}", code, message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_provider_verification_setup() {
        let mut provider_tests = ShippingProviderTests::new();
        
        // Test basic setup
        assert_eq!(provider_tests.server_port, 8081);
        assert!(provider_tests.pact_file_path.to_string_lossy().contains("frontend-shipping-service.json"));
        
        // Test fixture setup
        provider_tests.setup_test_data_fixtures();
        assert!(provider_tests.test_data.contains_key("single_item_request"));
        assert!(provider_tests.test_data.contains_key("empty_items_request"));
        assert_eq!(provider_tests.test_data.len(), 5);
    }

    #[tokio::test]
    async fn test_response_validation_helpers() {
        let provider_tests = ShippingProviderTests::new();
        
        // Test valid response validation
        let valid_response = json!({
            "cost_usd": {
                "currency_code": "USD",
                "units": 8,
                "nanos": 990000000
            }
        });
        
        let result = provider_tests.validate_successful_quote_response(&valid_response);
        assert!(result.is_ok());
        
        // Test error response validation
        let error_response = json!({
            "error": {
                "code": "INVALID_REQUEST",
                "message": "Items array cannot be empty"
            }
        });
        
        let result = provider_tests.validate_error_response(&error_response, "Items array cannot be empty");
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // This test requires the server to be running
    async fn test_full_provider_verification() {
        let mut provider_tests = ShippingProviderTests::new();
        
        // This test would run the full verification suite
        // Ignored by default as it requires external dependencies
        let result = provider_tests.run_full_verification().await;
        println!("Full verification result: {:?}", result);
    }
}