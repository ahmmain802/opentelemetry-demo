use reqwest;
use serde_json::json;
use std::error::Error;

/// Client for publishing verification results to Pact Broker
pub struct PactBrokerClient {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

impl PactBrokerClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:9292".to_string(),
            username: "pact_broker".to_string(),
            password: "pact_broker".to_string(),
        }
    }

    /// Publish verification results to Pact Broker
    pub async fn publish_verification_result(
        &self,
        consumer: &str,
        provider: &str,
        consumer_version: &str,
        provider_version: &str,
        success: bool,
        test_results: Vec<TestResult>,
    ) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        
        let verification_result = json!({
            "success": success,
            "providerApplicationVersion": provider_version,
            "testResults": test_results.iter().map(|r| json!({
                "interactionDescription": r.interaction_description,
                "success": r.success,
                "mismatches": r.mismatches
            })).collect::<Vec<_>>()
        });

        let url = format!(
            "{}/pacts/provider/{}/consumer/{}/pact-version/{}/verification-results",
            self.base_url, provider, consumer, consumer_version
        );

        println!("📤 Publishing verification results to Pact Broker...");
        println!("   URL: {}", url);
        println!("   Success: {}", success);
        println!("   Test Results: {}", test_results.len());

        let response = client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .header("Content-Type", "application/json")
            .json(&verification_result)
            .send()
            .await?;

        if response.status().is_success() {
            println!("✅ Successfully published verification results to Pact Broker");
            println!("🌐 View results at: {}/pacts/provider/{}/consumer/{}", 
                self.base_url, provider, consumer);
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Unable to read response".to_string());
            println!("❌ Failed to publish verification results: HTTP {}", status);
            println!("   Response: {}", body);
            return Err(format!("HTTP {}: {}", status, body).into());
        }

        Ok(())
    }

    /// Check if Pact Broker is available
    pub async fn health_check(&self) -> bool {
        let client = reqwest::Client::new();
        let url = format!("{}/diagnostic/status/heartbeat", self.base_url);
        
        match client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub interaction_description: String,
    pub success: bool,
    pub mismatches: Vec<String>,
}

impl TestResult {
    pub fn new(description: &str, success: bool) -> Self {
        Self {
            interaction_description: description.to_string(),
            success,
            mismatches: Vec::new(),
        }
    }

    pub fn with_mismatch(mut self, mismatch: &str) -> Self {
        self.mismatches.push(mismatch.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pact_broker_client_creation() {
        let client = PactBrokerClient::new();
        assert_eq!(client.base_url, "http://localhost:9292");
        assert_eq!(client.username, "pact_broker");
        assert_eq!(client.password, "pact_broker");
    }

    #[test]
    fn test_test_result_creation() {
        let result = TestResult::new("test interaction", true);
        assert_eq!(result.interaction_description, "test interaction");
        assert!(result.success);
        assert!(result.mismatches.is_empty());

        let result_with_mismatch = TestResult::new("failed interaction", false)
            .with_mismatch("Expected 200 but got 500");
        assert!(!result_with_mismatch.success);
        assert_eq!(result_with_mismatch.mismatches.len(), 1);
    }
}