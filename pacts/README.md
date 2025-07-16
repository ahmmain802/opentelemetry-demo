# Pact Files Directory

This directory contains the generated Pact contract files that define the interactions between consumer and provider services.

## Structure

- `consumer-contracts/` - Generated pact files from consumer tests
- `provider-verification/` - Provider verification results and logs
- `shared/` - Shared utilities and configuration for contract testing

## Pact Files

### Frontend → Shipping Service
- File: `consumer-contracts/frontend-shipping.json`
- Consumer: Frontend Service (TypeScript/Next.js)
- Provider: Shipping Service (Rust/Actix)
- Protocol: HTTP/JSON

### Checkout → Accounting Service  
- File: `consumer-contracts/checkout-accounting.json`
- Consumer: Checkout Service (Go)
- Provider: Accounting Service (C#/.NET)
- Protocol: Kafka Messages (Protobuf)

## Usage

1. Run consumer tests to generate pact files in `consumer-contracts/`
2. Run provider verification tests to validate against consumer contracts
3. Provider verification results are stored in `provider-verification/`

## Configuration

See `shared/pact-config.yml` for shared Pact configuration settings.