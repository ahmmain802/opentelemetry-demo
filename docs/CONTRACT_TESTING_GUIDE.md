# Contract Testing Guide for Junior Developers

## Table of Contents
1. [What is Contract Testing?](#what-is-contract-testing)
2. [Why Use Contract Testing?](#why-use-contract-testing)
3. [How Pact Works](#how-pact-works)
4. [Setup Guide](#setup-guide)
5. [Writing Your First Contract Test](#writing-your-first-contract-test)
6. [Best Practices](#best-practices)
7. [Common Patterns](#common-patterns)
8. [Troubleshooting](#troubleshooting)
9. [Advanced Topics](#advanced-topics)

## What is Contract Testing?

Contract testing is a technique for testing the integration points between services in a microservices architecture. Instead of testing services together (integration testing), contract testing allows you to test each service in isolation while ensuring they can communicate correctly.

### Real-World Analogy
Think of contract testing like a business contract between two companies:
- **Company A** (Consumer) needs specific services from **Company B** (Provider)
- They write a contract that specifies exactly what Company A expects
- Company B must fulfill their obligations according to this contract
- If either side changes their requirements or capabilities, the contract helps identify issues

### In Software Terms
- **Consumer**: The service that makes requests (e.g., Frontend calling an API)
- **Provider**: The service that responds to requests (e.g., Backend API)
- **Contract**: A specification of the expected interactions between consumer and provider

## Why Use Contract Testing?

### Problems Contract Testing Solves

1. **Integration Test Hell**: Traditional integration tests are slow, flaky, and hard to maintain
2. **Breaking Changes**: Services can break each other without anyone knowing until production
3. **Test Environment Complexity**: Setting up all services for integration testing is complex
4. **Feedback Speed**: Finding integration issues late in the development cycle

### Benefits

✅ **Fast Feedback**: Tests run in seconds, not minutes  
✅ **Reliable**: No flaky network calls or service dependencies  
✅ **Independent Development**: Teams can work independently  
✅ **Clear Contracts**: Explicit documentation of service interactions  
✅ **Confidence**: Deploy knowing your services will work together  

## How Pact Works

Pact follows a simple two-step process:

### Step 1: Consumer Tests (Contract Generation)
```
Consumer Test → Mock Provider → Generate Pact File
```

1. Consumer writes tests against a mock provider
2. Mock provider records all interactions
3. Pact file is generated containing the contract

### Step 2: Provider Verification (Contract Validation)
```
Pact File → Real Provider → Verification Results
```

1. Provider reads the pact file
2. Provider replays all interactions against the real service
3. Verification passes/fails based on whether provider meets contract

### Visual Flow
```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Consumer   │───▶│  Pact File   │───▶│  Provider   │
│    Test     │    │  (Contract)  │    │Verification │
└─────────────┘    └──────────────┘    └─────────────┘
```

## Setup Guide

### Prerequisites

Before starting, ensure you have these tools installed:

```bash
# Check if tools are installed
node --version    # Should be 18+
go version       # Should be 1.24+
cargo --version  # Should be 1.70+
dotnet --version # Should be 8.0+
```

### Quick Setup

1. **Clone and Navigate**
   ```bash
   git clone <repository>
   cd opentelemetry-demo
   ```

2. **Run Setup Script**
   ```bash
   ./pacts/run-contract-tests.sh --help
   ```

3. **Install Dependencies**
   ```bash
   ./pacts/run-contract-tests.sh --skip-deps=false
   ```

### Manual Setup (If Script Fails)

#### Frontend (TypeScript)
```bash
cd src/frontend
npm install @pact-foundation/pact jest ts-jest
```

#### Checkout Service (Go)
```bash
cd src/checkout
go get github.com/pact-foundation/pact-go/v2
```

#### Shipping Service (Rust)
```bash
cd src/shipping
# Add to Cargo.toml [dev-dependencies]
# pact_verifier = "1.2.8"
cargo build
```

#### Accounting Service (C#)
```bash
cd src/accounting
dotnet add package PactNet
```

## Writing Your First Contract Test

Let's walk through creating a simple contract test step by step.

### Example: Frontend → Shipping Service

#### Step 1: Consumer Test (Frontend)

Create `src/frontend/__tests__/contracts/shipping.consumer.test.ts`:

```typescript
import { PactV3, MatchersV3 } from '@pact-foundation/pact';
import { PactTestHelper } from '../../../pacts/shared/test-utils';

const { like, eachLike } = MatchersV3;

describe('Frontend → Shipping Service Contract', () => {
  // Set up Pact mock provider
  const provider = PactTestHelper.setupPactProvider(
    'Frontend',
    'Shipping', 
    3001
  );

  describe('GET /shipping-quote', () => {
    it('should return shipping cost for valid request', async () => {
      // Define what we expect from the provider
      await provider
        .given('products exist in catalog')
        .uponReceiving('a request for shipping quote')
        .withRequest({
          method: 'POST',
          path: '/shipping-quote',
          headers: { 'Content-Type': 'application/json' },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(2)
            }, { min: 1 }),
            address: {
              street_address: like('1600 Amphitheatre Parkway'),
              city: like('Mountain View'),
              state: like('CA'),
              country: like('United States'),
              zip_code: like('94043')
            }
          }
        })
        .willRespondWith({
          status: 200,
          headers: { 'Content-Type': 'application/json' },
          body: {
            cost_usd: {
              currency_code: like('USD'),
              units: like(8),
              nanos: like(990000000)
            }
          }
        });

      // Execute the interaction
      await provider.executeTest(async (mockService) => {
        // Your actual frontend code would make this request
        const response = await fetch(`${mockService.url}/shipping-quote`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            items: [{ product_id: 'OLJCESPC7Z', quantity: 2 }],
            address: {
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: 'CA',
              country: 'United States',
              zip_code: '94043'
            }
          })
        });

        const data = await response.json();
        
        // Verify the response
        expect(response.status).toBe(200);
        expect(data.cost_usd.currency_code).toBe('USD');
        expect(typeof data.cost_usd.units).toBe('number');
      });
    });
  });
});
```

#### Step 2: Run Consumer Test

```bash
cd src/frontend
npm run test:pact:consumer
```

This generates: `pacts/consumer-contracts/frontend-shipping.json`

#### Step 3: Provider Verification (Shipping Service)

Create `src/shipping/tests/pact_verification.rs`:

```rust
use pact_verifier::*;
use crate::pact_utils::PactTestHelper;

#[tokio::test]
async fn verify_frontend_contract() {
    let helper = PactTestHelper::new();
    
    // Load the pact file generated by consumer
    let pact_files = helper
        .load_pact_files("frontend", "shipping")
        .expect("Failed to load pact files");

    // Set up provider info
    let provider = helper.setup_provider_verification(
        "Shipping",
        "http://localhost:8080"
    );

    // Run verification
    let result = verify_provider(
        provider,
        pact_files,
        &helper.setup_provider_state("products exist in catalog")
    ).await;

    assert!(result.is_ok(), "Provider verification failed: {:?}", result);
}
```

#### Step 4: Run Provider Verification

```bash
cd src/shipping
cargo test --test pact_verification
```

## Best Practices

### 1. Contract Design Principles

#### ✅ DO: Focus on Behavior, Not Implementation
```typescript
// Good - Tests the behavior
.willRespondWith({
  status: 200,
  body: {
    user_id: like('12345'),
    name: like('John Doe'),
    email: like('john@example.com')
  }
})

// Bad - Too specific about implementation
.willRespondWith({
  status: 200,
  body: {
    user_id: '12345',
    name: 'John Doe',
    email: 'john@example.com',
    created_at: '2024-01-15T10:30:00Z',
    last_login: '2024-01-16T14:22:33Z'
  }
})
```

#### ✅ DO: Use Appropriate Matchers
```typescript
// Use type matchers for flexible contracts
body: {
  id: like(123),                    // Any number
  name: like('Product Name'),       // Any string
  price: like(29.99),              // Any number
  tags: eachLike(like('tag'), {min: 1}), // Array with at least 1 item
  sku: regex('^[A-Z]{3}\\d{3}$', 'ABC123') // Specific format
}
```

#### ❌ DON'T: Over-specify Contracts
```typescript
// Bad - Too rigid
body: {
  id: 123,
  name: 'Exact Product Name',
  description: 'Exact description with specific wording...'
}
```

### 2. Test Organization

#### ✅ DO: Group Related Interactions
```typescript
describe('User Service Contract', () => {
  describe('User Management', () => {
    it('should create user')
    it('should get user by id')
    it('should update user')
  })
  
  describe('Authentication', () => {
    it('should login user')
    it('should refresh token')
  })
})
```

#### ✅ DO: Use Descriptive Test Names
```typescript
// Good
it('should return 400 when email format is invalid')
it('should return user profile when valid token provided')

// Bad
it('should work')
it('test user endpoint')
```

### 3. Provider States

#### ✅ DO: Use Clear Provider States
```typescript
// Consumer test
.given('user with id 123 exists')
.given('user with id 123 has premium subscription')

// Provider verification
setupProviderState('user with id 123 exists', () => {
  // Create test user in database
  createTestUser({ id: 123, name: 'Test User' })
})
```

#### ✅ DO: Clean Up After Tests
```typescript
afterEach(() => {
  // Clean up test data
  cleanupTestUsers()
  resetDatabase()
})
```

### 4. Error Handling

#### ✅ DO: Test Error Scenarios
```typescript
describe('Error Handling', () => {
  it('should return 404 when user not found', async () => {
    await provider
      .given('user with id 999 does not exist')
      .uponReceiving('request for non-existent user')
      .withRequest({
        method: 'GET',
        path: '/users/999'
      })
      .willRespondWith({
        status: 404,
        body: {
          error: like('User not found'),
          code: like('USER_NOT_FOUND')
        }
      })
  })
})
```

### 5. Versioning Strategy

#### ✅ DO: Version Your Contracts
```bash
# Tag consumer versions
git tag consumer-v1.2.3
./pacts/run-contract-tests.sh --consumer-version=1.2.3

# Use semantic versioning
# Major: Breaking changes
# Minor: New features (backward compatible)
# Patch: Bug fixes
```

#### ✅ DO: Maintain Backward Compatibility
```typescript
// When adding new fields, make them optional
body: {
  id: like(123),
  name: like('Product'),
  // New field - optional for backward compatibility
  category: like('Electronics').optional()
}
```

## Common Patterns

### 1. Authentication Patterns

#### API Key Authentication
```typescript
.withRequest({
  method: 'GET',
  path: '/protected-resource',
  headers: {
    'Authorization': like('Bearer abc123token'),
    'X-API-Key': like('api-key-value')
  }
})
```

#### JWT Token Pattern
```typescript
.given('user is authenticated')
.withRequest({
  headers: {
    'Authorization': regex(
      '^Bearer [A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+$',
      'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
    )
  }
})
```

### 2. Pagination Patterns

```typescript
// Request with pagination
.withRequest({
  method: 'GET',
  path: '/users',
  query: {
    page: like('1'),
    limit: like('10'),
    sort: like('name')
  }
})
.willRespondWith({
  body: {
    data: eachLike({
      id: like(1),
      name: like('User Name')
    }),
    pagination: {
      page: like(1),
      limit: like(10),
      total: like(100),
      has_next: like(true)
    }
  }
})
```

### 3. Message Queue Patterns (Kafka)

```go
// Go consumer test for Kafka messages
func TestOrderMessage(t *testing.T) {
    helper := NewPactTestHelper()
    
    // Define expected message structure
    expectedMessage := helper.GetOrderMessageMatchers()
    
    // Create message pact
    messagePact := consumer.NewMessagePact(consumer.Config{
        Consumer: "Checkout",
        Provider: "Accounting",
        PactDir:  helper.PactDir,
    })
    
    messagePact.
        Given("order is ready for processing").
        ExpectsToReceive("order processing message").
        WithContent(expectedMessage).
        AsType(&OrderMessage{})
}
```

### 4. File Upload Patterns

```typescript
.withRequest({
  method: 'POST',
  path: '/upload',
  headers: {
    'Content-Type': regex(
      '^multipart/form-data; boundary=',
      'multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW'
    )
  },
  body: like('file content placeholder')
})
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Pact File Not Generated
**Problem**: Consumer test passes but no pact file is created.

**Solutions**:
```bash
# Check pact directory exists
ls -la pacts/consumer-contracts/

# Verify pact configuration
cat pacts/shared/pact-config.yml

# Run with debug logging
DEBUG=pact* npm run test:pact:consumer
```

#### 2. Provider Verification Fails
**Problem**: Provider can't verify the contract.

**Solutions**:
```bash
# Check if provider is running
curl http://localhost:8080/health

# Verify pact file structure
cat pacts/consumer-contracts/frontend-shipping.json | jq .

# Check provider state setup
# Ensure provider states match consumer expectations
```

#### 3. Matcher Issues
**Problem**: Tests fail due to incorrect matchers.

**Common Fixes**:
```typescript
// Wrong - too specific
body: { id: 123 }

// Right - flexible
body: { id: like(123) }

// Wrong - exact array
body: { items: ['item1', 'item2'] }

// Right - flexible array
body: { items: eachLike(like('item'), {min: 1}) }
```

#### 4. Network/Timeout Issues
**Problem**: Tests timeout or fail intermittently.

**Solutions**:
```typescript
// Increase timeout
const provider = new PactV3({
  timeout: 30000, // 30 seconds
  // ... other config
})

// Add retry logic
jest.retryTimes(3)
```

### Debug Commands

```bash
# Validate all pact files
find pacts/consumer-contracts -name "*.json" -exec jq . {} \;

# Check test output
./pacts/run-contract-tests.sh --consumer-only 2>&1 | tee test-output.log

# Verify provider endpoints
curl -X POST http://localhost:8080/shipping-quote \
  -H "Content-Type: application/json" \
  -d '{"items":[{"product_id":"TEST","quantity":1}]}'
```

## Advanced Topics

### 1. Consumer-Driven Contract Evolution

As your services evolve, follow this process:

1. **Consumer Changes First**
   ```typescript
   // Add new optional field
   body: {
     existing_field: like('value'),
     new_field: like('new_value').optional()
   }
   ```

2. **Update Provider**
   ```rust
   // Provider adds support for new field
   #[derive(Serialize)]
   struct Response {
       existing_field: String,
       #[serde(skip_serializing_if = "Option::is_none")]
       new_field: Option<String>,
   }
   ```

3. **Make Field Required (Later)**
   ```typescript
   // After all consumers updated
   body: {
     existing_field: like('value'),
     new_field: like('new_value') // No longer optional
   }
   ```

### 2. Cross-Language Testing

When services use different languages:

```yaml
# CI Pipeline example
test-contracts:
  steps:
    - name: Run TypeScript Consumer Tests
      run: cd frontend && npm run test:pact
    
    - name: Run Go Consumer Tests  
      run: cd checkout && go test ./contracts/...
    
    - name: Run Rust Provider Verification
      run: cd shipping && cargo test --test pact_verification
    
    - name: Run C# Provider Verification
      run: cd accounting && dotnet test --filter PactVerification
```

### 3. Pact Broker Integration

For team collaboration:

```bash
# Publish contracts to broker
pact-broker publish pacts/consumer-contracts \
  --consumer-app-version 1.2.3 \
  --broker-base-url https://your-pact-broker.com

# Verify against broker
pact-broker can-i-deploy \
  --pacticipant Frontend \
  --version 1.2.3 \
  --to production
```

### 4. Performance Considerations

```typescript
// Batch related tests
describe('User API Performance', () => {
  beforeAll(async () => {
    // Setup once for all tests
    await setupTestData()
  })
  
  // Multiple interactions in one test
  it('should handle user lifecycle', async () => {
    await provider
      .addInteraction(createUserInteraction)
      .addInteraction(getUserInteraction)  
      .addInteraction(updateUserInteraction)
      .executeTest(async (mockService) => {
        // Test all interactions
      })
  })
})
```

## Next Steps

1. **Start Small**: Begin with one simple consumer-provider pair
2. **Add Gradually**: Expand to more complex interactions
3. **Automate**: Integrate with your CI/CD pipeline
4. **Monitor**: Set up alerts for contract violations
5. **Educate**: Share knowledge with your team

## Resources

- [Pact Documentation](https://docs.pact.io/)
- [Contract Testing Best Practices](https://pactflow.io/blog/contract-testing-best-practices/)
- [Microservices Testing Strategies](https://martinfowler.com/articles/microservice-testing/)

Remember: Contract testing is a journey, not a destination. Start simple, learn from experience, and gradually build more sophisticated contracts as your understanding grows.