# Contract Testing Example Walkthrough

This document walks through a complete example of implementing contract testing between the Frontend and Shipping services, from start to finish.

## 📋 Scenario

**Business Requirement**: The Frontend needs to get shipping quotes from the Shipping service.

**Services Involved**:
- **Consumer**: Frontend (TypeScript/Next.js)
- **Provider**: Shipping Service (Rust/Actix)

**API Contract**:
- **Endpoint**: `POST /shipping-quote`
- **Purpose**: Calculate shipping cost for cart items

## 🎯 Step 1: Understand the Requirements

### What the Frontend Needs
```typescript
// Frontend wants to call:
const response = await fetch('/api/shipping-quote', {
  method: 'POST',
  body: JSON.stringify({
    items: [
      { product_id: 'OLJCESPC7Z', quantity: 2 },
      { product_id: '66VCHSJNUP', quantity: 1 }
    ],
    address: {
      street_address: '1600 Amphitheatre Parkway',
      city: 'Mountain View',
      state: 'CA',
      country: 'United States',
      zip_code: '94043'
    }
  })
});

// And expects back:
{
  cost_usd: {
    currency_code: 'USD',
    units: 8,
    nanos: 990000000
  }
}
```

### What the Shipping Service Provides
```rust
// Shipping service has endpoint:
#[post("/shipping-quote")]
async fn get_shipping_quote(request: ShippingRequest) -> ShippingResponse {
    // Calculate shipping cost
    ShippingResponse {
        cost_usd: Money {
            currency_code: "USD".to_string(),
            units: 8,
            nanos: 990000000,
        }
    }
}
```

## 🔨 Step 2: Write the Consumer Test

Create `src/frontend/__tests__/contracts/shipping.consumer.test.ts`:

```typescript
import { PactV3, MatchersV3 } from '@pact-foundation/pact';
import { PactTestHelper, TestDataFactory } from '../../../pacts/shared/test-utils';

const { like, eachLike } = MatchersV3;

describe('Frontend → Shipping Service Contract', () => {
  let provider: PactV3;

  beforeAll(() => {
    // Set up mock provider on port 3001
    provider = PactTestHelper.setupPactProvider('Frontend', 'Shipping', 3001);
  });

  afterAll(async () => {
    // Clean up
    await provider.finalize();
  });

  describe('POST /shipping-quote', () => {
    it('should return shipping cost for valid cart items', async () => {
      // Step 2a: Define the interaction
      await provider
        .given('products exist in catalog')  // Provider state
        .uponReceiving('a request for shipping quote with valid items')
        .withRequest({
          method: 'POST',
          path: '/shipping-quote',
          headers: {
            'Content-Type': 'application/json'
          },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(2)
            }, { min: 1 }),
            address: PactTestHelper.getAddressMatchers()
          }
        })
        .willRespondWith({
          status: 200,
          headers: {
            'Content-Type': 'application/json'
          },
          body: {
            cost_usd: PactTestHelper.getShippingMatchers().costUsd
          }
        });

      // Step 2b: Execute the test
      await provider.executeTest(async (mockService) => {
        // This is where your actual frontend code would run
        const testData = TestDataFactory.getValidShippingRequest();
        
        const response = await fetch(`${mockService.url}/shipping-quote`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(testData)
        });

        const data = await response.json();

        // Verify the response matches expectations
        expect(response.status).toBe(200);
        expect(data.cost_usd).toBeDefined();
        expect(data.cost_usd.currency_code).toBe('USD');
        expect(typeof data.cost_usd.units).toBe('number');
        expect(typeof data.cost_usd.nanos).toBe('number');
      });
    });

    it('should return 400 for empty cart', async () => {
      await provider
        .given('service is available')
        .uponReceiving('a request with empty cart')
        .withRequest({
          method: 'POST',
          path: '/shipping-quote',
          headers: {
            'Content-Type': 'application/json'
          },
          body: {
            items: [],
            address: PactTestHelper.getAddressMatchers()
          }
        })
        .willRespondWith({
          status: 400,
          headers: {
            'Content-Type': 'application/json'
          },
          body: PactTestHelper.getErrorMatchers().badRequest.body
        });

      await provider.executeTest(async (mockService) => {
        const testData = TestDataFactory.getEmptyCartRequest();
        
        const response = await fetch(`${mockService.url}/shipping-quote`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(testData)
        });

        expect(response.status).toBe(400);
        
        const data = await response.json();
        expect(data.error).toBeDefined();
      });
    });
  });
});
```

## 🏃 Step 3: Run the Consumer Test

```bash
cd src/frontend
npm run test:pact:consumer
```

**Expected Output**:
```
✓ Frontend → Shipping Service Contract
  ✓ should return shipping cost for valid cart items
  ✓ should return 400 for empty cart

Pact file written to: ../../../pacts/consumer-contracts/frontend-shipping.json
```

**Generated Pact File** (`pacts/consumer-contracts/frontend-shipping.json`):
```json
{
  "consumer": {
    "name": "Frontend"
  },
  "provider": {
    "name": "Shipping"
  },
  "interactions": [
    {
      "description": "a request for shipping quote with valid items",
      "providerState": "products exist in catalog",
      "request": {
        "method": "POST",
        "path": "/shipping-quote",
        "headers": {
          "Content-Type": "application/json"
        },
        "body": {
          "items": [
            {
              "product_id": "OLJCESPC7Z",
              "quantity": 2
            }
          ],
          "address": {
            "street_address": "1600 Amphitheatre Parkway",
            "city": "Mountain View",
            "state": "CA",
            "country": "United States",
            "zip_code": "94043"
          }
        },
        "matchingRules": {
          "$.body.items": {
            "matchers": [{"match": "type", "min": 1}]
          },
          "$.body.items[*].product_id": {
            "matchers": [{"match": "type"}]
          }
        }
      },
      "response": {
        "status": 200,
        "headers": {
          "Content-Type": "application/json"
        },
        "body": {
          "cost_usd": {
            "currency_code": "USD",
            "units": 8,
            "nanos": 990000000
          }
        },
        "matchingRules": {
          "$.body.cost_usd.units": {
            "matchers": [{"match": "type"}]
          }
        }
      }
    }
  ],
  "metadata": {
    "pactSpecification": {
      "version": "3.0.0"
    }
  }
}
```

## 🔍 Step 4: Implement Provider Verification

Create `src/shipping/tests/pact_verification.rs`:

```rust
use actix_web::{test, web, App};
use pact_verifier::*;
use serde_json::json;
use std::collections::HashMap;

use crate::pact_utils::{PactTestHelper, TestDataFactory};
use shipping::shipping_service::{get_shipping_quote, ShippingRequest};

#[tokio::test]
async fn verify_frontend_shipping_contract() {
    let helper = PactTestHelper::new();
    
    // Step 4a: Load the pact file
    let pact_files = helper
        .load_pact_files("frontend", "shipping")
        .expect("Failed to load pact files");

    // Step 4b: Set up the real provider
    let provider_info = ProviderInfo {
        name: "Shipping".to_string(),
        host: "localhost".to_string(),
        port: Some(8080),
        path: "/".to_string(),
        protocol: "http".to_string(),
        ..Default::default()
    };

    // Step 4c: Set up provider states
    let mut provider_states = HashMap::new();
    
    provider_states.insert(
        "products exist in catalog".to_string(),
        Box::new(|| {
            // Set up test data - products exist in catalog
            setup_test_products();
            Ok(())
        }) as Box<dyn Fn() -> Result<(), String>>
    );

    provider_states.insert(
        "service is available".to_string(),
        Box::new(|| {
            // Ensure service is ready
            Ok(())
        }) as Box<dyn Fn() -> Result<(), String>>
    );

    // Step 4d: Run verification
    let verification_result = verify_provider(
        provider_info,
        pact_files,
        &provider_states,
    ).await;

    // Step 4e: Assert verification passed
    match verification_result {
        Ok(_) => println!("✅ Provider verification passed!"),
        Err(e) => {
            panic!("❌ Provider verification failed: {:?}", e);
        }
    }
}

// Helper function to set up test data
fn setup_test_products() {
    // In a real implementation, this might:
    // - Insert test products into database
    // - Set up mock external services
    // - Configure test environment
    println!("Setting up test products in catalog");
}

#[tokio::test]
async fn test_shipping_endpoint_directly() {
    // Step 4f: Direct unit test of the endpoint
    let app = test::init_service(
        App::new().route("/shipping-quote", web::post().to(get_shipping_quote))
    ).await;

    let test_request = TestDataFactory::get_valid_shipping_request();
    
    let req = test::TestRequest::post()
        .uri("/shipping-quote")
        .set_json(&test_request)
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("cost_usd").is_some());
}
```

## 🚀 Step 5: Run Provider Verification

```bash
cd src/shipping

# Start the shipping service (in another terminal)
cargo run

# Run verification tests
cargo test --test pact_verification
```

**Expected Output**:
```
running 2 tests
test verify_frontend_shipping_contract ... ok
test test_shipping_endpoint_directly ... ok

✅ Provider verification passed!

Verification results written to: ../../pacts/provider-verification/shipping-verification.log
```

## 🔄 Step 6: Run Complete Workflow

```bash
# Run everything together
./pacts/run-contract-tests.sh
```

**Expected Output**:
```
🚀 Starting Contract Testing Workflow
Project root: /path/to/opentelemetry-demo

[INFO] Checking prerequisites...
✅ All prerequisites satisfied

[INFO] Installing dependencies...
✅ Dependencies installed

[INFO] Running consumer contract tests...
[INFO] Running frontend consumer tests...
✅ Frontend consumer tests passed

[INFO] Running provider verification tests...
[INFO] Running shipping provider verification...
✅ Shipping provider verification passed

[INFO] Validating generated pact files...
✅ Found pact file: frontend-shipping.json

[INFO] Generating test report...
✅ Test report generated: pacts/contract-test-report.md

✅ Contract testing completed successfully in 45s
```

## 📊 Step 7: Review Generated Artifacts

### Pact File
```bash
cat pacts/consumer-contracts/frontend-shipping.json | jq .
```

### Verification Log
```bash
cat pacts/provider-verification/shipping-verification.log
```

### Test Report
```bash
cat pacts/contract-test-report.md
```

## 🐛 Step 8: Troubleshooting Common Issues

### Issue 1: Consumer Test Fails
```
Error: Request failed with status 404
```

**Solution**: Check the mock provider setup
```typescript
// Make sure port and path are correct
const provider = PactTestHelper.setupPactProvider('Frontend', 'Shipping', 3001);

// Verify the request path matches
.withRequest({
  path: '/shipping-quote'  // Must match exactly
})
```

### Issue 2: No Pact File Generated
```
Test passes but no pact file created
```

**Solution**: Check pact directory and permissions
```bash
# Create directory if missing
mkdir -p pacts/consumer-contracts

# Check permissions
ls -la pacts/consumer-contracts/

# Verify pact configuration
cat pacts/shared/pact-config.yml
```

### Issue 3: Provider Verification Fails
```
Error: Connection refused to localhost:8080
```

**Solution**: Ensure provider service is running
```bash
# Start shipping service
cd src/shipping
cargo run &

# Verify it's running
curl http://localhost:8080/health

# Then run verification
cargo test --test pact_verification
```

### Issue 4: Matcher Mismatch
```
Error: Expected type String but got Number
```

**Solution**: Use appropriate matchers
```typescript
// Wrong - too specific
body: { quantity: 2 }

// Right - flexible
body: { quantity: like(2) }
```

## 🎉 Step 9: Success Criteria

You've successfully implemented contract testing when:

✅ **Consumer test passes** and generates pact file  
✅ **Provider verification passes** against the contract  
✅ **Both services can be developed independently**  
✅ **Contract changes are detected automatically**  
✅ **Tests run fast** (< 30 seconds total)  

## 🔄 Step 10: Next Steps

1. **Add More Scenarios**
   - Invalid address format
   - Network timeout handling
   - Authentication errors

2. **Integrate with CI/CD**
   ```yaml
   # .github/workflows/contract-tests.yml
   - name: Run Contract Tests
     run: ./pacts/run-contract-tests.sh
   ```

3. **Set Up Monitoring**
   - Alert on contract test failures
   - Track contract evolution over time

4. **Expand to Other Services**
   - Checkout → Accounting (Kafka messages)
   - Frontend → Product Catalog
   - Payment → Fraud Detection

## 💡 Key Takeaways

1. **Consumer-Driven**: The consumer defines what they need
2. **Fast Feedback**: Tests run in seconds, not minutes
3. **Independent Development**: Teams don't block each other
4. **Living Documentation**: Contracts serve as API documentation
5. **Confidence**: Deploy knowing services will work together

This example demonstrates the complete contract testing workflow. The pattern can be applied to any consumer-provider relationship in your microservices architecture.