# OpenTelemetry Demo Makefile
# Includes contract testing targets for easy development

.PHONY: help
help: ## Show this help message
	@echo "OpenTelemetry Demo - Available Commands"
	@echo "======================================"
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Contract Testing (Local)
.PHONY: contracts-validate
contracts-validate: ## Validate contract testing setup
	./pacts/validate-setup.sh

.PHONY: contracts-test
contracts-test: ## Run all contract tests locally
	./pacts/run-contract-tests.sh

.PHONY: contracts-consumer
contracts-consumer: ## Run consumer tests only (generates contracts)
	./pacts/run-contract-tests.sh --consumer-only

.PHONY: contracts-provider
contracts-provider: ## Run provider verification only
	./pacts/run-contract-tests.sh --provider-only

##@ Development
.PHONY: dev-setup
dev-setup: ## Set up development environment
	@echo "Setting up development environment..."
	make contracts-validate

.PHONY: dev-test
dev-test: ## Run contract tests locally
	@echo "🏠 Running contract tests locally"
	make contracts-test

##@ Utilities
.PHONY: clean
clean: ## Clean up generated files
	@echo "Cleaning up..."
	rm -rf pacts/consumer-contracts/*.json
	rm -rf pacts/provider-verification/*.log
	@echo "✅ Cleanup completed"

.PHONY: docs
docs: ## Open documentation
	@echo "📚 Available documentation:"
	@echo "  - Getting Started: docs/GETTING_STARTED.md"
	@echo "  - Complete Guide: docs/CONTRACT_TESTING_GUIDE.md"
	@echo "  - Quick Reference: docs/PACT_QUICK_REFERENCE.md"
	@echo "  - Example Walkthrough: docs/EXAMPLE_WALKTHROUGH.md"


.PHONY: status
status: ## Show contract testing status
	@echo "🔍 Contract Testing Status"
	@echo "========================="
	@echo ""
	@echo "📁 Generated Pact Files:"
	@ls -la pacts/consumer-contracts/ 2>/dev/null | grep "\.json$$" || echo "  No pact files found"
	@echo ""
	@echo "📋 Verification Logs:"
	@ls -la pacts/provider-verification/ 2>/dev/null | grep "\.log$$" || echo "  No verification logs found"
	@echo ""
	@echo "🔧 Environment Status:"
	@command -v node >/dev/null 2>&1 && echo "  ✅ Node.js available" || echo "  ❌ Node.js not found"
	@command -v go >/dev/null 2>&1 && echo "  ✅ Go available" || echo "  ❌ Go not found"
	@command -v cargo >/dev/null 2>&1 && echo "  ✅ Rust available" || echo "  ❌ Rust not found"
	@command -v dotnet >/dev/null 2>&1 && echo "  ✅ .NET available" || echo "  ⚠️  .NET not found (optional)"

##@ Quick Start for Junior Developers
.PHONY: quickstart
quickstart: ## Complete setup and first test run for new developers
	@echo "🚀 Quick Start for Contract Testing"
	@echo "=================================="
	@echo ""
	@echo "Step 1: Validating environment..."
	make contracts-validate
	@echo ""
	@echo "Step 2: Running first contract test..."
	make contracts-consumer
	@echo ""
	@echo "🎉 Quick start completed!"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Check generated contracts: make status"
	@echo "  2. Read the docs: make docs"
	@echo "  3. Run full test suite: make contracts-test"

##@ Troubleshooting
.PHONY: debug
debug: ## Debug contract testing issues
	@echo "🐛 Contract Testing Debug Information"
	@echo "===================================="
	@echo ""
	@echo "Environment:"
	@echo "  OS: $$(uname -s)"
	@echo "  Architecture: $$(uname -m)"
	@echo ""
	@echo "Tools:"
	@command -v node >/dev/null 2>&1 && echo "  Node.js: $$(node --version)" || echo "  Node.js: Not found"
	@command -v npm >/dev/null 2>&1 && echo "  npm: $$(npm --version)" || echo "  npm: Not found"
	@command -v go >/dev/null 2>&1 && echo "  Go: $$(go version | awk '{print $$3}')" || echo "  Go: Not found"
	@command -v cargo >/dev/null 2>&1 && echo "  Rust: $$(rustc --version | awk '{print $$2}')" || echo "  Rust: Not found"
	@command -v dotnet >/dev/null 2>&1 && echo "  .NET: $$(dotnet --version)" || echo "  .NET: Not found"
	@command -v docker >/dev/null 2>&1 && echo "  Docker: $$(docker --version | awk '{print $$3}' | sed 's/,//')" || echo "  Docker: Not found"
	@echo ""
	@echo "Project Structure:"
	@ls -la pacts/ 2>/dev/null || echo "  pacts/ directory not found"
	@echo ""
	@echo "Recent Logs:"
	@tail -n 5 pacts/provider-verification/*.log 2>/dev/null || echo "  No recent verification logs"

# Default target
.DEFAULT_GOAL := help