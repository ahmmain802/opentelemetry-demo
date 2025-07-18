# Contract Testing with Pact

This document describes the contract testing implementation for the OpenTelemetry Demo using Pact.

## Overview

Contract testing ensures API compatibility between consumer and provider services by:
- Consumers define their expectations of provider APIs
- Providers verify they meet those expectations
- Tests run independently without requiring both services to be running simultaneously

## Quick Start

### Run Complete Contract Testing Workflow

```bash
# Single command to run everything
./run-full-contract-testing.sh
```

This script will:
1. Start Pact Broker (if not running)
2. Publish consumer contracts
3. Run provider verification tests
4. Publish verification results
5. Show contract testing matrix and summary

### View Results
- **Pact Broker UI**: http://localhost:9292
- **Matrix View**: http://localhost:9292/matrix
- **Consumer**: http://localhost:9292/pacticipants/frontend
- **Provider**: http://localhost:9292/pacticipants/shipping-service

## Architecture

The implementation covers:

**Frontend → Shipping Service** (HTTP/JSON)
- Consumer: Frontend (TypeScript/Next.js)
- Provider: Shipping Service (Rust/Actix)
- Protocol: HTTP POST to `/get-quote`

## Contract Details

### Frontend → Shipping Service Contract

**Request Format**:
```json
{
  "items": [
    {"product_id": "OLJCESPC7Z", "quantity": 2}
  ],
  "address": {
    "street_address": "1600 Amphitheatre Parkway",
    "city": "Mountain View",
    "state": "CA",
    "country": "United States",
    "zip_code": "94043"
  }
}
```

**Response Format**:
```json
{
  "cost_usd": {
    "currency_code": "USD",
    "units": 8,
    "nanos": 990000000
  }
}
```

**Contract Expectations**:
- Returns 200 OK for valid requests
- Returns 400 Bad Request for empty items array
- Returns 400 Bad Request for missing address fields
- Response always includes `cost_usd` with USD currency
- Supports international addresses

## Implementation Status

### Completed ✅
- Pact Broker setup with Docker Compose
- Frontend consumer contract tests
- Shipping service provider verification tests
- Contract violation detection and reporting
- Automated publishing to Pact Broker
- Complete workflow automation

### Contract Violations Found

The current implementation has identified **7 contract violations**:

1. **Data Structure Mismatches**:
   - Shipping service expects only `quantity` but consumer sends `product_id`
   - Shipping service expects only `zip_code` but consumer sends full address

2. **Validation Issues**:
   - Service returns 500 errors instead of 400 for validation failures
   - Missing request validation for empty items and invalid data

These violations demonstrate that contract testing is working correctly by identifying real API compatibility issues.

## Files and Directories

```
├── run-full-contract-testing.sh     # Main workflow script
├── docker-compose.pact-broker.yml   # Pact Broker setup
├── CONTRACT_TESTING_EXECUTION_REPORT.md  # Detailed results
├── src/shipping/tests/               # Provider verification tests
└── pacts/consumer-contracts/         # Consumer contract files
```

## Troubleshooting

### Common Issues

1. **Pact Broker Connection Failed**:
   ```bash
   curl http://localhost:9292  # Check if broker is running
   ```

2. **Contract Verification Failures**:
   - Review violation messages in test output
   - Check provider implementation matches consumer expectations

### Getting Help

- Check the Pact documentation: https://docs.pact.io/
- Review `CONTRACT_TESTING_EXECUTION_REPORT.md` for detailed analysis
- Use Pact Broker UI to visualize contract relationships