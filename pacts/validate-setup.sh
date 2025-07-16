#!/bin/bash

# Contract Testing Setup Validation Script
# This script helps junior developers verify their environment is correctly configured

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Emojis for better UX
CHECK="✅"
CROSS="❌"
WARNING="⚠️"
INFO="ℹ️"
ROCKET="🚀"

print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}  Contract Testing Setup Validator${NC}"
    echo -e "${PURPLE}================================${NC}"
    echo ""
}

print_section() {
    echo -e "${BLUE}📋 $1${NC}"
    echo "---"
}

print_success() {
    echo -e "${GREEN}${CHECK} $1${NC}"
}

print_error() {
    echo -e "${RED}${CROSS} $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}${WARNING} $1${NC}"
}

print_info() {
    echo -e "${BLUE}${INFO} $1${NC}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get version of a command
get_version() {
    case $1 in
        "node")
            node --version 2>/dev/null | sed 's/v//'
            ;;
        "npm")
            npm --version 2>/dev/null
            ;;
        "go")
            go version 2>/dev/null | awk '{print $3}' | sed 's/go//'
            ;;
        "cargo")
            cargo --version 2>/dev/null | awk '{print $2}'
            ;;
        "rustc")
            rustc --version 2>/dev/null | awk '{print $2}'
            ;;
        "dotnet")
            dotnet --version 2>/dev/null
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Function to compare versions
version_ge() {
    printf '%s\n%s\n' "$2" "$1" | sort -V -C
}

# Check prerequisites
check_prerequisites() {
    print_section "Checking Prerequisites"
    
    local all_good=true
    
    # Node.js
    if command_exists node; then
        local node_version=$(get_version node)
        if version_ge "$node_version" "18.0.0"; then
            print_success "Node.js $node_version (required: 18+)"
        else
            print_error "Node.js $node_version is too old (required: 18+)"
            all_good=false
        fi
    else
        print_error "Node.js not found (required for frontend tests)"
        print_info "Install from: https://nodejs.org/"
        all_good=false
    fi
    
    # npm
    if command_exists npm; then
        local npm_version=$(get_version npm)
        print_success "npm $npm_version"
    else
        print_error "npm not found (usually comes with Node.js)"
        all_good=false
    fi
    
    # Go
    if command_exists go; then
        local go_version=$(get_version go)
        if version_ge "$go_version" "1.21.0"; then
            print_success "Go $go_version (required: 1.21+)"
        else
            print_error "Go $go_version is too old (required: 1.21+)"
            all_good=false
        fi
    else
        print_error "Go not found (required for checkout tests)"
        print_info "Install from: https://golang.org/dl/"
        all_good=false
    fi
    
    # Rust
    if command_exists cargo; then
        local cargo_version=$(get_version cargo)
        local rust_version=$(get_version rustc)
        print_success "Rust $rust_version with Cargo $cargo_version"
    else
        print_error "Rust/Cargo not found (required for shipping tests)"
        print_info "Install from: https://rustup.rs/"
        all_good=false
    fi
    
    # .NET
    if command_exists dotnet; then
        local dotnet_version=$(get_version dotnet)
        if version_ge "$dotnet_version" "8.0.0"; then
            print_success ".NET $dotnet_version (required: 8.0+)"
        else
            print_error ".NET $dotnet_version is too old (required: 8.0+)"
            all_good=false
        fi
    else
        print_error ".NET not found (required for accounting tests)"
        print_info "Install from: https://dotnet.microsoft.com/download"
        all_good=false
    fi
    
    # Optional tools
    if command_exists jq; then
        print_success "jq (for JSON validation)"
    else
        print_warning "jq not found (optional, for debugging pact files)"
        print_info "Install with: brew install jq (macOS) or apt-get install jq (Ubuntu)"
    fi
    
    echo ""
    
    if [ "$all_good" = true ]; then
        print_success "All prerequisites satisfied!"
        return 0
    else
        print_error "Some prerequisites are missing. Please install them before continuing."
        return 1
    fi
}

# Check project structure
check_project_structure() {
    print_section "Checking Project Structure"
    
    local all_good=true
    
    # Check main directories
    local required_dirs=(
        "pacts"
        "pacts/consumer-contracts"
        "pacts/provider-verification"
        "pacts/shared"
        "src/frontend/__tests__/contracts"
        "src/checkout/contracts"
        "src/shipping/tests"
        "src/accounting/Tests"
    )
    
    for dir in "${required_dirs[@]}"; do
        if [ -d "$PROJECT_ROOT/$dir" ]; then
            print_success "Directory exists: $dir"
        else
            print_error "Missing directory: $dir"
            all_good=false
        fi
    done
    
    # Check key files
    local required_files=(
        "pacts/run-contract-tests.sh"
        "pacts/shared/pact-config.yml"
        "pacts/shared/test-utils.ts"
        "src/checkout/contracts/test_utils.go"
        "src/shipping/tests/pact_utils.rs"
        "src/accounting/Tests/PactTestHelper.cs"
        "CONTRACT_TESTING.md"
    )
    
    for file in "${required_files[@]}"; do
        if [ -f "$PROJECT_ROOT/$file" ]; then
            print_success "File exists: $file"
        else
            print_error "Missing file: $file"
            all_good=false
        fi
    done
    
    echo ""
    
    if [ "$all_good" = true ]; then
        print_success "Project structure is correct!"
        return 0
    else
        print_error "Project structure is incomplete."
        return 1
    fi
}

# Check dependencies
check_dependencies() {
    print_section "Checking Dependencies"
    
    local all_good=true
    
    # Frontend dependencies
    print_info "Checking frontend dependencies..."
    if [ -f "$PROJECT_ROOT/src/frontend/package.json" ]; then
        if grep -q "@pact-foundation/pact" "$PROJECT_ROOT/src/frontend/package.json"; then
            print_success "Frontend: @pact-foundation/pact found"
        else
            print_error "Frontend: @pact-foundation/pact not found in package.json"
            all_good=false
        fi
        
        if grep -q "jest" "$PROJECT_ROOT/src/frontend/package.json"; then
            print_success "Frontend: jest found"
        else
            print_error "Frontend: jest not found in package.json"
            all_good=false
        fi
    else
        print_error "Frontend: package.json not found"
        all_good=false
    fi
    
    # Go dependencies
    print_info "Checking Go dependencies..."
    if [ -f "$PROJECT_ROOT/src/checkout/go.mod" ]; then
        if grep -q "pact-foundation/pact-go" "$PROJECT_ROOT/src/checkout/go.mod"; then
            print_success "Checkout: pact-go found"
        else
            print_error "Checkout: pact-go not found in go.mod"
            all_good=false
        fi
    else
        print_error "Checkout: go.mod not found"
        all_good=false
    fi
    
    # Rust dependencies
    print_info "Checking Rust dependencies..."
    if [ -f "$PROJECT_ROOT/src/shipping/Cargo.toml" ]; then
        if grep -q "pact_verifier" "$PROJECT_ROOT/src/shipping/Cargo.toml"; then
            print_success "Shipping: pact_verifier found"
        else
            print_error "Shipping: pact_verifier not found in Cargo.toml"
            all_good=false
        fi
    else
        print_error "Shipping: Cargo.toml not found"
        all_good=false
    fi
    
    # .NET dependencies
    print_info "Checking .NET dependencies..."
    if [ -f "$PROJECT_ROOT/src/accounting/Accounting.csproj" ]; then
        if grep -q "PactNet" "$PROJECT_ROOT/src/accounting/Accounting.csproj"; then
            print_success "Accounting: PactNet found"
        else
            print_error "Accounting: PactNet not found in Accounting.csproj"
            all_good=false
        fi
    else
        print_error "Accounting: Accounting.csproj not found"
        all_good=false
    fi
    
    echo ""
    
    if [ "$all_good" = true ]; then
        print_success "All dependencies are configured!"
        return 0
    else
        print_error "Some dependencies are missing."
        return 1
    fi
}

# Check scripts and permissions
check_scripts() {
    print_section "Checking Scripts and Permissions"
    
    local all_good=true
    
    # Check main script
    if [ -f "$PROJECT_ROOT/pacts/run-contract-tests.sh" ]; then
        if [ -x "$PROJECT_ROOT/pacts/run-contract-tests.sh" ]; then
            print_success "run-contract-tests.sh is executable"
        else
            print_warning "run-contract-tests.sh is not executable"
            print_info "Fix with: chmod +x pacts/run-contract-tests.sh"
        fi
        
        # Test script help
        if "$PROJECT_ROOT/pacts/run-contract-tests.sh" --help >/dev/null 2>&1; then
            print_success "run-contract-tests.sh --help works"
        else
            print_error "run-contract-tests.sh --help failed"
            all_good=false
        fi
    else
        print_error "run-contract-tests.sh not found"
        all_good=false
    fi
    
    # Check package.json scripts
    if [ -f "$PROJECT_ROOT/src/frontend/package.json" ]; then
        if grep -q "test:pact" "$PROJECT_ROOT/src/frontend/package.json"; then
            print_success "Frontend: pact test scripts configured"
        else
            print_error "Frontend: pact test scripts not found"
            all_good=false
        fi
    fi
    
    echo ""
    
    if [ "$all_good" = true ]; then
        print_success "Scripts are properly configured!"
        return 0
    else
        print_error "Some scripts need attention."
        return 1
    fi
}

# Provide recommendations
provide_recommendations() {
    print_section "Recommendations for Junior Developers"
    
    echo -e "${BLUE}📚 Learning Resources:${NC}"
    echo "• Read: docs/CONTRACT_TESTING_GUIDE.md (comprehensive tutorial)"
    echo "• Quick ref: docs/PACT_QUICK_REFERENCE.md (commands & templates)"
    echo "• Example: docs/EXAMPLE_WALKTHROUGH.md (step-by-step)"
    echo ""
    
    echo -e "${BLUE}🛠️ Development Workflow:${NC}"
    echo "1. Start with consumer tests (define what you need)"
    echo "2. Generate pact files (contracts)"
    echo "3. Implement provider verification (validate contracts)"
    echo "4. Run full test suite before committing"
    echo ""
    
    echo -e "${BLUE}🔧 Useful Commands:${NC}"
    echo "• ./pacts/run-contract-tests.sh --help"
    echo "• ./pacts/run-contract-tests.sh --consumer-only"
    echo "• ./pacts/run-contract-tests.sh --provider-only"
    echo ""
    
    echo -e "${BLUE}🐛 When Things Go Wrong:${NC}"
    echo "• Check logs in pacts/provider-verification/"
    echo "• Validate pact files with: cat pacts/consumer-contracts/*.json | jq ."
    echo "• Ensure services are running before provider verification"
    echo "• Use shared test utilities for consistent patterns"
    echo ""
}

# Main execution
main() {
    print_header
    
    local overall_status=0
    
    # Run all checks
    if ! check_prerequisites; then
        overall_status=1
    fi
    
    if ! check_project_structure; then
        overall_status=1
    fi
    
    if ! check_dependencies; then
        overall_status=1
    fi
    
    if ! check_scripts; then
        overall_status=1
    fi
    
    provide_recommendations
    
    # Final status
    echo -e "${PURPLE}================================${NC}"
    if [ $overall_status -eq 0 ]; then
        echo -e "${GREEN}${ROCKET} Setup validation passed!${NC}"
        echo -e "${GREEN}You're ready to start contract testing!${NC}"
        echo ""
        echo -e "${BLUE}Next steps:${NC}"
        echo "1. Read the documentation: docs/CONTRACT_TESTING_GUIDE.md"
        echo "2. Try the example: docs/EXAMPLE_WALKTHROUGH.md"
        echo "3. Run your first test: ./pacts/run-contract-tests.sh --help"
    else
        echo -e "${RED}${CROSS} Setup validation failed!${NC}"
        echo -e "${RED}Please fix the issues above before proceeding.${NC}"
        echo ""
        echo -e "${BLUE}Need help?${NC}"
        echo "• Check the troubleshooting section in CONTRACT_TESTING.md"
        echo "• Review the setup guide in docs/CONTRACT_TESTING_GUIDE.md"
        echo "• Ask for help in your team's Slack channel"
    fi
    echo -e "${PURPLE}================================${NC}"
    
    exit $overall_status
}

# Run main function
main "$@"