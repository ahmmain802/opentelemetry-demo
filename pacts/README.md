# Pact Contract Testing

This directory contains contract testing files for the OpenTelemetry Demo.

## Files

- **consumer-contracts/**: Generated pact files from consumer tests
- **shared/**: Shared configuration and utilities

## Usage

Run the complete contract testing workflow:

```bash
# From project root
./run-full-contract-testing.sh
```

## Generated Contracts

- **frontend-shipping-service.json**: Contract between frontend and shipping service