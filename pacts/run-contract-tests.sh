#!/bin/bash

# Master script to run all contract tests
# This script runs consumer tests first, then provider verification tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 Starting Contract Testing Workflow"
echo "Project root: $PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites based on what tests we're running
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    local missing_tools=()
    
    # Always check for npm (frontend consumer tests)
    if ! command_exists npm; then
        missing_tools+=("npm")
    fi
    
    # Check Go if running consumer tests or all tests
    if [[ "$CONSUMER_ONLY" == "true" || "$PROVIDER_ONLY" != "true" ]]; then
        if ! command_exists go; then
            missing_tools+=("go")
        fi
    fi
    
    # Check Rust if running provider tests or all tests
    if [[ "$PROVIDER_ONLY" == "true" || "$CONSUMER_ONLY" != "true" ]]; then
        if ! command_exists cargo; then
            missing_tools+=("cargo")
        fi
    fi
    
    # Check .NET if running provider tests or all tests
    if [[ "$PROVIDER_ONLY" == "true" || "$CONSUMER_ONLY" != "true" ]]; then
        if ! command_exists dotnet; then
            missing_tools+=("dotnet")
        fi
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        if [[ "$CONSUMER_ONLY" == "true" ]]; then
            print_warning "Running consumer-only tests, some missing tools may be OK"
        else
            exit 1
        fi
    fi
    
    print_success "Prerequisites check completed"
}

# Install dependencies
install_dependencies() {
    print_status "Installing dependencies..."
    
    # Frontend dependencies
    print_status "Installing frontend dependencies..."
    cd "$PROJECT_ROOT/src/frontend"
    npm install
    
    # Go dependencies
    print_status "Installing Go dependencies..."
    cd "$PROJECT_ROOT/src/checkout"
    go mod tidy
    
    # Rust dependencies
    print_status "Installing Rust dependencies..."
    cd "$PROJECT_ROOT/src/shipping"
    cargo build --dev
    
    # C# dependencies
    print_status "Installing C# dependencies..."
    cd "$PROJECT_ROOT/src/accounting"
    dotnet restore
    
    print_success "Dependencies installed"
}

# Run consumer tests
run_consumer_tests() {
    print_status "Running consumer contract tests..."
    
    local consumer_failures=0
    
    # Frontend consumer tests
    print_status "Running frontend consumer tests..."
    cd "$PROJECT_ROOT/src/frontend"
    if npm run test:pact:consumer; then
        print_success "Frontend consumer tests passed"
    else
        print_error "Frontend consumer tests failed"
        ((consumer_failures++))
    fi
    
    # Checkout consumer tests
    print_status "Running checkout consumer tests..."
    cd "$PROJECT_ROOT/src/checkout"
    if go test -v ./contracts/...; then
        print_success "Checkout consumer tests passed"
    else
        print_error "Checkout consumer tests failed"
        ((consumer_failures++))
    fi
    
    if [ $consumer_failures -eq 0 ]; then
        print_success "All consumer tests passed"
        return 0
    else
        print_error "$consumer_failures consumer test suite(s) failed"
        return 1
    fi
}

# Run provider verification tests
run_provider_tests() {
    print_status "Running provider verification tests..."
    
    local provider_failures=0
    
    # Shipping provider tests
    print_status "Running shipping provider verification..."
    cd "$PROJECT_ROOT/src/shipping"
    if cargo test --test pact_verification; then
        print_success "Shipping provider verification passed"
    else
        print_error "Shipping provider verification failed"
        ((provider_failures++))
    fi
    
    # Accounting provider tests
    print_status "Running accounting provider verification..."
    cd "$PROJECT_ROOT/src/accounting"
    if dotnet test --filter "Category=PactVerification"; then
        print_success "Accounting provider verification passed"
    else
        print_error "Accounting provider verification failed"
        ((provider_failures++))
    fi
    
    if [ $provider_failures -eq 0 ]; then
        print_success "All provider verification tests passed"
        return 0
    else
        print_error "$provider_failures provider verification suite(s) failed"
        return 1
    fi
}

# Validate generated pact files
validate_pact_files() {
    print_status "Validating generated pact files..."
    
    local pact_dir="$PROJECT_ROOT/pacts/consumer-contracts"
    local expected_files=("frontend-shipping.json" "checkout-accounting.json")
    
    for file in "${expected_files[@]}"; do
        if [ -f "$pact_dir/$file" ]; then
            print_success "Found pact file: $file"
        else
            print_warning "Missing pact file: $file"
        fi
    done
}

# Generate test report
generate_report() {
    print_status "Generating test report..."
    
    local report_file="$PROJECT_ROOT/pacts/contract-test-report.md"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    cat > "$report_file" << EOF
# Contract Test Report

**Generated:** $timestamp

## Test Results

### Consumer Tests
- Frontend → Shipping: $([[ -f "$PROJECT_ROOT/pacts/consumer-contracts/frontend-shipping.json" ]] && echo "✅ PASSED" || echo "❌ FAILED")
- Checkout → Accounting: $([[ -f "$PROJECT_ROOT/pacts/consumer-contracts/checkout-accounting.json" ]] && echo "✅ PASSED" || echo "❌ FAILED")

### Provider Verification
- Shipping Service: $([ -f "$PROJECT_ROOT/pacts/provider-verification/shipping-verification.log" ] && echo "✅ PASSED" || echo "❌ FAILED")
- Accounting Service: $([ -f "$PROJECT_ROOT/pacts/provider-verification/accounting-verification.log" ] && echo "✅ PASSED" || echo "❌ FAILED")

## Generated Pact Files

$(ls -la "$PROJECT_ROOT/pacts/consumer-contracts/" 2>/dev/null | grep "\.json$" || echo "No pact files found")

## Verification Logs

$(ls -la "$PROJECT_ROOT/pacts/provider-verification/" 2>/dev/null | grep "\.log$" || echo "No verification logs found")

EOF

    print_success "Test report generated: $report_file"
}

# Main execution
main() {
    local start_time=$(date +%s)
    
    # Parse command line arguments
    local skip_deps=false
    local consumer_only=false
    local provider_only=false
    
    # Set global variables for prerequisite checking
    CONSUMER_ONLY=false
    PROVIDER_ONLY=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-deps)
                skip_deps=true
                shift
                ;;
            --consumer-only)
                consumer_only=true
                CONSUMER_ONLY=true
                shift
                ;;
            --provider-only)
                provider_only=true
                PROVIDER_ONLY=true
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --skip-deps      Skip dependency installation"
                echo "  --consumer-only  Run only consumer tests"
                echo "  --provider-only  Run only provider verification tests"
                echo "  -h, --help       Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    # Create necessary directories
    mkdir -p "$PROJECT_ROOT/pacts/consumer-contracts"
    mkdir -p "$PROJECT_ROOT/pacts/provider-verification"
    
    check_prerequisites
    
    if [ "$skip_deps" = false ]; then
        install_dependencies
    fi
    
    local exit_code=0
    
    if [ "$provider_only" = false ]; then
        if ! run_consumer_tests; then
            exit_code=1
        fi
    fi
    
    if [ "$consumer_only" = false ] && [ $exit_code -eq 0 ]; then
        if ! run_provider_tests; then
            exit_code=1
        fi
    fi
    
    validate_pact_files
    generate_report
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    if [ $exit_code -eq 0 ]; then
        print_success "Contract testing completed successfully in ${duration}s"
    else
        print_error "Contract testing failed after ${duration}s"
    fi
    
    exit $exit_code
}

# Run main function with all arguments
main "$@"