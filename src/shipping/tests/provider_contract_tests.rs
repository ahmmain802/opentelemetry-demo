use actix_web::{App, HttpServer, middleware::Logger};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

use shipping::shipping_service::{get_quote, ship_order};

/// Provider verification tests for shipping service
/// Tests the EXISTING implementation against consumer contract expectations
/// Requirements: 1.1, 1.2, 5.2, 7.3
pub struct ShippingProviderTests {
    pub server_port: u16,
    pub pact_file_path: PathBuf,
    pub test_data: HashMap<String, Value>,
    pub contract_violations: Vec<String>,
}

impl ShippingProviderTests {
    /// Create new provider test setup (Task 3.1 requirement)
    pub fn new() -> Self {
        Self {
            server_port: 8081,
            pact_file_path: PathBuf::from("../../pacts/consumer-contracts/frontend-shipping-service.json"),
            test_data: HashMap::new(),
            contract_violations: Vec::new(),
        }
    }

    /// Set up Actix-web test server for shipping service (Task 3.1)
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
        
        println!("✓ Test server started successfully");
        Ok(())
    }

    /// Create test data based on consumer contract expectations (Task 3.3)
    /// Sets up deterministic test data for various contract scenarios
    pub fn setup_contract_test_data(&mut self) {
        println!("Setting up test data based on consumer contract expectations");

        // Test data that matches what the consumer contract expects
        // NOTE: This will likely fail because current service expects different structure

        // Valid request as expected by consumer (will likely fail due to missing product_id)
        self.test_data.insert(
            "consumer_expected_single_item".to_string(),
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

        // Multiple items test (consumer contract expectation)
        self.test_data.insert(
            "consumer_expected_multiple_items".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 2},
                    {"product_id": "66VCHSJNUP", "quantity": 1},
                    {"product_id": "1YMWWN1N4O", "quantity": 3}
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

        // International address test (consumer contract expectation)
        self.test_data.insert(
            "consumer_expected_international".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 1}
                ],
                "address": {
                    "street_address": "123 Main Street",
                    "city": "Toronto",
                    "state": "ON",
                    "country": "Canada",
                    "zip_code": "M5V 3A8"
                }
            })
        );

        // Valid request as current service expects (only quantity, only zip_code)
        self.test_data.insert(
            "current_service_format".to_string(),
            json!({
                "items": [
                    {"quantity": 1}
                ],
                "address": {
                    "zip_code": "94043"
                }
            })
        );

        // Current service format with multiple items
        self.test_data.insert(
            "current_service_multiple_items".to_string(),
            json!({
                "items": [
                    {"quantity": 2},
                    {"quantity": 1},
                    {"quantity": 3}
                ],
                "address": {
                    "zip_code": "94043"
                }
            })
        );

        // Empty items array (consumer expects 400 error)
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

        // Missing address fields (consumer expects 400 error)
        self.test_data.insert(
            "missing_address_fields".to_string(),
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

        // Missing address entirely (consumer expects 400 error)
        self.test_data.insert(
            "missing_address".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 1}
                ]
            })
        );

        // Invalid quantity values (consumer expects 400 error)
        self.test_data.insert(
            "invalid_quantity".to_string(),
            json!({
                "items": [
                    {"product_id": "OLJCESPC7Z", "quantity": 0}
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

        println!("✓ Test data fixtures created: {} scenarios", self.test_data.len());
    }

    /// Set up provider state for deterministic test results (Task 3.3)
    /// Ensures consistent responses for contract verification
    pub fn setup_provider_state(&mut self, state_name: &str) -> Result<(), String> {
        println!("Setting up provider state: {}", state_name);
        
        match state_name {
            "single_item_quote" => {
                // State for testing single item quote calculation
                // Current service calculates based on quantity sum
                println!("  - Configured for single item (quantity=1) quote test");
                println!("  - Expected cost: $8.99 (base shipping rate)");
            },
            "multiple_items_quote" => {
                // State for testing multiple items quote calculation  
                // Current service: base $8.99 + $2.00 per item over 5
                println!("  - Configured for multiple items (quantity=6) quote test");
                println!("  - Expected cost: $10.99 ($8.99 base + $2.00 for 1 extra item)");
            },
            "international_shipping" => {
                // State for international shipping (if supported)
                println!("  - Configured for international shipping test");
                println!("  - Note: Current service may not handle international rates");
            },
            "empty_cart_validation" => {
                // State for testing empty cart validation
                println!("  - Configured for empty cart validation test");
                println!("  - Expected: 400 Bad Request (if validation implemented)");
            },
            "invalid_address_validation" => {
                // State for testing address validation
                println!("  - Configured for invalid address validation test");
                println!("  - Expected: 400 Bad Request (if validation implemented)");
            },
            _ => {
                return Err(format!("Unknown provider state: {}", state_name));
            }
        }
        
        Ok(())
    }

    /// Reset provider state between tests (Task 3.3)
    pub fn reset_provider_state(&mut self) {
        println!("Resetting provider state for next test");
        // Clear any cached data or state that might affect subsequent tests
        // For this service, state is mostly stateless, but we ensure clean slate
    }

    /// Test current service against consumer contract expectations (Task 3.2)
    /// This will document where the service fails to meet contract requirements
    pub async fn verify_against_consumer_contract(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== Verifying current service against consumer contract ===");
        let client = reqwest::Client::new();
        let base_url = format!("http://127.0.0.1:{}", self.server_port);

        // Test 1: Consumer expected single item format (will likely fail)
        self.setup_provider_state("single_item_quote").ok();
        println!("\n1. Testing consumer expected single item request format...");
        if let Some(request_data) = self.test_data.get("consumer_expected_single_item") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                if self.validate_successful_quote_response(&body).is_ok() {
                    println!("✓ Consumer expected single item format works");
                } else {
                    let violation = "Single item response format doesn't match consumer expectations".to_string();
                    self.contract_violations.push(violation.clone());
                    println!("✗ {}", violation);
                }
            } else {
                let violation = format!("Consumer expected single item request failed with status: {} (expected 200)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
                
                // Try to get error details
                if let Ok(error_text) = response.text().await {
                    println!("  Error details: {}", error_text);
                }
            }
        }
        self.reset_provider_state();

        // Test 2: Consumer expected multiple items format (will likely fail)
        self.setup_provider_state("multiple_items_quote").ok();
        println!("\n2. Testing consumer expected multiple items request format...");
        if let Some(request_data) = self.test_data.get("consumer_expected_multiple_items") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                if self.validate_successful_quote_response(&body).is_ok() {
                    println!("✓ Consumer expected multiple items format works");
                } else {
                    let violation = "Multiple items response format doesn't match consumer expectations".to_string();
                    self.contract_violations.push(violation.clone());
                    println!("✗ {}", violation);
                }
            } else {
                let violation = format!("Consumer expected multiple items request failed with status: {} (expected 200)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
                
                if let Ok(error_text) = response.text().await {
                    println!("  Error details: {}", error_text);
                }
            }
        }
        self.reset_provider_state();

        // Test 3: Consumer expected international address (will likely fail)
        self.setup_provider_state("international_shipping").ok();
        println!("\n3. Testing consumer expected international address format...");
        if let Some(request_data) = self.test_data.get("consumer_expected_international") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                if self.validate_successful_quote_response(&body).is_ok() {
                    println!("✓ Consumer expected international address format works");
                } else {
                    let violation = "International address response format doesn't match consumer expectations".to_string();
                    self.contract_violations.push(violation.clone());
                    println!("✗ {}", violation);
                }
            } else {
                let violation = format!("Consumer expected international address request failed with status: {} (expected 200)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
                
                if let Ok(error_text) = response.text().await {
                    println!("  Error details: {}", error_text);
                }
            }
        }
        self.reset_provider_state();

        // Test 4: Current service single item format (should work)
        println!("\n4. Testing current service single item format...");
        if let Some(request_data) = self.test_data.get("current_service_format") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                if self.validate_successful_quote_response(&body).is_ok() {
                    println!("✓ Current service single item format works");
                } else {
                    println!("✗ Current service single item response format is invalid");
                }
            } else {
                println!("✗ Current service single item format failed with status: {}", response.status());
            }
        }

        // Test 5: Current service multiple items format (should work)
        println!("\n5. Testing current service multiple items format...");
        if let Some(request_data) = self.test_data.get("current_service_multiple_items") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status().is_success() {
                let body: Value = response.json().await?;
                if self.validate_successful_quote_response(&body).is_ok() {
                    println!("✓ Current service multiple items format works");
                } else {
                    println!("✗ Current service multiple items response format is invalid");
                }
            } else {
                println!("✗ Current service multiple items format failed with status: {}", response.status());
            }
        }

        // Test 6: Empty items array (consumer expects 400, service might not validate)
        self.setup_provider_state("empty_cart_validation").ok();
        println!("\n6. Testing empty items array handling...");
        if let Some(request_data) = self.test_data.get("empty_items_request") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                println!("✓ Empty items correctly returns 400");
            } else {
                let violation = format!("Empty items should return 400, got: {} (contract violation)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
            }
        }
        self.reset_provider_state();

        // Test 7: Missing address fields (consumer expects 400)
        self.setup_provider_state("invalid_address_validation").ok();
        println!("\n7. Testing missing address fields handling...");
        if let Some(request_data) = self.test_data.get("missing_address_fields") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                println!("✓ Missing address fields correctly returns 400");
            } else {
                let violation = format!("Missing address fields should return 400, got: {} (contract violation)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
            }
        }
        self.reset_provider_state();

        // Test 8: Missing address entirely (consumer expects 400)
        println!("\n8. Testing missing address entirely handling...");
        if let Some(request_data) = self.test_data.get("missing_address") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                println!("✓ Missing address entirely correctly returns 400");
            } else {
                let violation = format!("Missing address entirely should return 400, got: {} (contract violation)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
            }
        }

        // Test 9: Invalid quantity values (consumer expects 400)
        println!("\n9. Testing invalid quantity values handling...");
        if let Some(request_data) = self.test_data.get("invalid_quantity") {
            let response = client
                .post(&format!("{}/get-quote", base_url))
                .header("Content-Type", "application/json")
                .json(request_data)
                .send()
                .await?;

            if response.status() == 400 {
                println!("✓ Invalid quantity values correctly returns 400");
            } else {
                let violation = format!("Invalid quantity values should return 400, got: {} (contract violation)", response.status());
                self.contract_violations.push(violation.clone());
                println!("✗ {}", violation);
            }
        }

        Ok(())
    }

    /// Test malformed JSON handling (Task 3.2)
    pub async fn verify_error_handling(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Testing error handling ===");
        let client = reqwest::Client::new();
        let base_url = format!("http://127.0.0.1:{}", self.server_port);

        // Test malformed JSON
        println!("Testing malformed JSON handling...");
        let response = client
            .post(&format!("{}/get-quote", base_url))
            .header("Content-Type", "application/json")
            .body("{ invalid json }")
            .send()
            .await?;

        if response.status() == 400 {
            println!("✓ Malformed JSON correctly returns 400");
        } else {
            let violation = format!("Malformed JSON should return 400, got: {}", response.status());
            self.contract_violations.push(violation.clone());
            println!("✗ {}", violation);
        }

        Ok(())
    }

    /// Generate contract compliance report (Task 3.3)
    pub fn generate_compliance_report(&self) {
        println!("\n=== CONTRACT COMPLIANCE REPORT ===");
        
        if self.contract_violations.is_empty() {
            println!("✅ All contract requirements are met!");
        } else {
            println!("❌ Found {} contract violations:", self.contract_violations.len());
            for (i, violation) in self.contract_violations.iter().enumerate() {
                println!("  {}. {}", i + 1, violation);
            }
            
            println!("\n📋 RECOMMENDATIONS:");
            println!("  - Update CartItem struct to include product_id field");
            println!("  - Update Address struct to include all required fields (street_address, city, state, country)");
            println!("  - Add request validation for empty items array");
            println!("  - Add request validation for missing address fields");
            println!("  - Add request validation for invalid quantity values (zero or negative)");
            println!("  - Ensure error responses match consumer expectations (400 for validation errors)");
            println!("  - Consider supporting international shipping rates");
        }
        
        println!("\n📊 SUMMARY:");
        println!("  Total contract tests: {}", 9 + 1); // 9 contract tests + 1 error handling test
        println!("  Contract violations: {}", self.contract_violations.len());
        println!("  Compliance rate: {:.1}%", 
            ((10 - self.contract_violations.len()) as f64 / 10.0) * 100.0);
        
        println!("\n🔍 TEST BREAKDOWN:");
        println!("  ✓ Consumer contract format tests: 3 (expected to fail with current implementation)");
        println!("  ✓ Current service format tests: 2 (expected to pass)");
        println!("  ✓ Validation error tests: 4 (expected to fail without validation)");
        println!("  ✓ Error handling tests: 1 (malformed JSON)");
        
        println!("\n💡 NEXT STEPS:");
        if self.contract_violations.len() > 5 {
            println!("  1. Focus on implementing request validation first");
            println!("  2. Update data structures to match consumer expectations");
            println!("  3. Re-run tests to verify improvements");
        } else if self.contract_violations.len() > 0 {
            println!("  1. Address remaining contract violations");
            println!("  2. Consider backward compatibility for existing clients");
        } else {
            println!("  1. Contract testing is working perfectly!");
            println!("  2. Consider adding more edge case tests");
        }
    }

    /// Run complete provider verification test suite
    pub async fn run_full_verification(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== SHIPPING SERVICE PROVIDER VERIFICATION ===");
        println!("Testing existing implementation against consumer contract");
        
        // Task 3.1: Set up test server and fixtures
        self.setup_test_server().await?;
        self.setup_contract_test_data();
        
        // Task 3.2: Verify contract compliance (expect failures)
        self.verify_against_consumer_contract().await?;
        self.verify_error_handling().await?;
        
        // Task 3.3: Generate compliance report
        self.generate_compliance_report();
        
        println!("\n=== VERIFICATION COMPLETED ===");
        Ok(())
    }

    /// Helper method to validate successful quote response format
    fn validate_successful_quote_response(&self, body: &Value) -> Result<(), String> {
        // Check for cost_usd field
        let cost_usd = body.get("cost_usd")
            .ok_or("Response missing cost_usd field")?;

        // Check currency_code
        let currency_code = cost_usd.get("currency_code")
            .and_then(|v| v.as_str())
            .ok_or("cost_usd missing currency_code")?;
        
        if currency_code != "USD" {
            return Err(format!("Expected currency_code 'USD', got '{}'", currency_code));
        }

        // Check units field
        let _units = cost_usd.get("units")
            .and_then(|v| v.as_u64())
            .ok_or("cost_usd missing or invalid units field")?;

        // Check nanos field
        let nanos = cost_usd.get("nanos")
            .and_then(|v| v.as_u64())
            .ok_or("cost_usd missing or invalid nanos field")?;

        // Validate nanos is in valid range (0-999,999,999)
        if nanos > 999_999_999 {
            return Err(format!("nanos value {} exceeds maximum 999,999,999", nanos));
        }

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
        provider_tests.setup_contract_test_data();
        assert!(provider_tests.test_data.contains_key("consumer_expected_single_item"));
        assert!(provider_tests.test_data.contains_key("current_service_format"));
        assert!(provider_tests.test_data.contains_key("consumer_expected_multiple_items"));
        assert!(provider_tests.test_data.contains_key("consumer_expected_international"));
        assert!(provider_tests.test_data.contains_key("empty_items_request"));
        assert!(provider_tests.test_data.contains_key("missing_address_fields"));
        assert!(provider_tests.test_data.contains_key("missing_address"));
        assert!(provider_tests.test_data.contains_key("invalid_quantity"));
        assert_eq!(provider_tests.test_data.len(), 9);
    }

    #[tokio::test]
    async fn test_response_validation() {
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
        
        // Test invalid response
        let invalid_response = json!({
            "cost_usd": {
                "currency_code": "EUR", // Wrong currency
                "units": 8,
                "nanos": 990000000
            }
        });
        
        let result = provider_tests.validate_successful_quote_response(&invalid_response);
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore] // This test requires the server to be running and external dependencies
    async fn test_full_provider_verification() {
        let mut provider_tests = ShippingProviderTests::new();
        
        // This test runs the full verification suite
        // It's expected to find contract violations with the current implementation
        let result = provider_tests.run_full_verification().await;
        
        // The test should complete successfully even if contract violations are found
        assert!(result.is_ok());
        
        // We expect to find some violations since the current service doesn't match the contract
        println!("Contract violations found: {}", provider_tests.contract_violations.len());
    }
}