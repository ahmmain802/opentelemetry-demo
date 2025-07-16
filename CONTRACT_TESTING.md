# Contract Testing Setup

This document describes the Pact contract testing infrastructure for the OpenTelemetry Demo microservices.

## 📚 Documentation

- **[🚀 Getting Started](docs/GETTING_STARTED.md)** - Quick start guide for new developers
- **[📖 Complete Guide](docs/CONTRACT_TESTING_GUIDE.md)** - Comprehensive tutorial with examples and best practices
- **[🐳 Containerized Testing](docs/CONTAINERIZED_TESTING.md)** - Docker-based testing guide
- **[⚡ Quick Reference](docs/PACT_QUICK_REFERENCE.md)** - Commands, templates, and troubleshooting
- **[🎯 Example Walkthrough](docs/EXAMPLE_WALKTHROUGH.md)** - Step-by-step implementation example

### 🎓 For Junior Developers

**New to contract testing?** Start here:
1. **[🚀 Getting Started Guide](docs/GETTING_STARTED.md)** - 5-minute setup + learning path
2. **[Validate your setup](pacts/validate-setup.sh)** - Check your environment
3. **[Follow the example](docs/EXAMPLE_WALKTHROUGH.md)** - Hands-on tutorial

## Overview

Contract testing is implemented using Pact for two critical service interactions:

1. **Frontend → Shipping Service**: HTTP/JSON communication for shipping quotes
2. **Checkout → Accounting Service**: Kafka message-based communication for order processing

## 🚀 Quick Start

### Option 1: Containerized (Recommended for Junior Developers)

**Prerequisites**: Just Docker Desktop

```bash
# Complete setup and first test (recommended for beginners)
make quickstart

# Or step by step:
make contracts-docker-build    # Build testing environment
make contracts-docker-test     # Run all tests
```

### Option 2: Local Development

**Prerequisites**: Node.js 18+, Go 1.24+, Rust 1.70+, .NET 8.0+

```bash
# Validate your local environment first
make contracts-validate

# Run all tests locally
make contracts-test
```

### Quick Commands

```bash
# Using Make (recommended)
make contracts-test              # Run all tests (auto-detects Docker)
make contracts-consumer          # Run consumer tests only
make contracts-provider          # Run provider verification only
make status                      # Check current status

# Using scripts directly
./pacts/run-contract-tests.sh           # Local testing
./pacts/run-contract-tests-docker.sh    # Containerized testing
```

### Run Individual Service Tests

#### Frontend Consumer Tests
```bash
cd src/frontend
npm install
npm run test:pact:consumer
```

#### Checkout Consumer Tests
```bash
cd src/checkout
go mod tidy
go test -v ./contracts/...
```

#### Shipping Provider Verification
```bash
cd src/shipping
cargo test --test pact_verification
```

#### Accounting Provider Verification
```bash
cd src/accounting
dotnet test --filter "Category=PactVerification"
```

## Directory Structure

```
pacts/
├── consumer-contracts/          # Generated pact files
│   ├── frontend-shipping.json
│   └── checkout-accounting.json
├── provider-verification/       # Verification logs
├── shared/                     # Shared utilities
│   ├── pact-config.yml        # Configuration
│   └── test-utils.ts          # TypeScript utilities
└── run-contract-tests.sh      # Master test script
```

## Service-Specific Setup

### Frontend (TypeScript/Jest)
- **Dependencies**: `@pact-foundation/pact`, `jest`, `ts-jest`
- **Test Location**: `src/frontend/__tests__/contracts/`
- **Configuration**: `jest.config.js`

### Checkout (Go)
- **Dependencies**: `github.com/pact-foundation/pact-go/v2`
- **Test Location**: `src/checkout/contracts/`
- **Utilities**: `test_utils.go`

### Shipping (Rust/Cargo)
- **Dependencies**: `pact_verifier`, `tokio-test`
- **Test Location**: `src/shipping/tests/`
- **Utilities**: `pact_utils.rs`

### Accounting (C#/.NET)
- **Dependencies**: `PactNet`, `xunit`
- **Test Location**: `src/accounting/Tests/`
- **Utilities**: `PactTestHelper.cs`

## Contract Definitions

### Frontend → Shipping Service

**Endpoint**: `POST /get-quote`

**Request**:
```json
{
  "items": [{"product_id": "string", "quantity": number}],
  "address": {
    "street_address": "string",
    "city": "string", 
    "state": "string",
    "country": "string",
    "zip_code": "string"
  }
}
```

**Response**:
```json
{
  "cost_usd": {
    "currency_code": "USD",
    "units": number,
    "nanos": number
  }
}
```

### Checkout → Accounting Service

**Topic**: `orders`

**Message** (Protobuf OrderResult):
```protobuf
message OrderResult {
  string order_id = 1;
  string shipping_tracking_id = 2;
  Money shipping_cost = 3;
  Address shipping_address = 4;
  repeated OrderItem items = 5;
}
```

## Troubleshooting

### Common Issues

1. **Pact files not generated**: Check consumer test execution and file permissions
2. **Provider verification fails**: Ensure provider service is running and accessible
3. **Dependency conflicts**: Run `npm install`, `go mod tidy`, `cargo update`, or `dotnet restore`

### Debug Commands

```bash
# Check generated pact files
ls -la pacts/consumer-contracts/

# View verification logs
cat pacts/provider-verification/*.log

# Validate pact file structure
cat pacts/consumer-contracts/frontend-shipping.json | jq .
```

### Test Data

All services use consistent test data defined in shared utilities:

- **Product IDs**: `OLJCESPC7Z`, `66VCHSJNUP`
- **Test Address**: 1600 Amphitheatre Parkway, Mountain View, CA 94043
- **Currency**: USD only
- **Tracking ID Format**: `Z[A-Z0-9]{8}`

## Next Steps

After setting up the infrastructure:

1. Implement consumer contract tests (Task 2)
2. Implement provider verification tests (Tasks 3, 5)
3. Add comprehensive test scenarios (Tasks 4, 6)
4. Create documentation and examples (Task 7)
5. Validate end-to-end workflow (Task 8)

## Support

For issues with contract testing setup:

1. Check service-specific README files
2. Review generated logs in `pacts/provider-verification/`
3. Validate pact file structure in `pacts/consumer-contracts/`
4. Ensure all prerequisites are installed and up-to-date