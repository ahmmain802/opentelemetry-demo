#!/bin/bash

echo "🔍 Validating Contract Testing Setup..."
echo "======================================"

# Check if we're in the right directory
if [ ! -f "docker-compose.yml" ]; then
    echo "❌ Please run this script from the opentelemetry-demo root directory"
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check directory structure
check_directories() {
    echo "📁 Checking directory structure..."
    
    local dirs=(
        "pacts"
        "pacts/consumer-contracts"
        "pacts/provider-verification"
        "pacts/shared"
        "src/frontend/__tests__/contracts"
        "src/checkout/contracts"
        "src/shipping/tests"
        "src/accounting/Tests"
    )
    
    for dir in "${dirs[@]}"; do
        if [ -d "$dir" ]; then
            echo "  ✅ $dir"
        else
            echo "  ❌ $dir (missing)"
            return 1
        fi
    done
    return 0
}

# Function to check key files
check_files() {
    echo "📄 Checking key files..."
    
    local files=(
        "pacts/README.md"
        "pacts/run-contract-tests.sh"
        "pacts/validate-setup.sh"
        "pacts/shared/pact-config.yml"
        "pacts/shared/test-utils.ts"
        "src/frontend/jest.config.js"
        "src/frontend/jest.setup.js"
        "src/shipping/tests/pact_utils.rs"
        "src/checkout/contracts/test_utils.go"
        "src/accounting/Tests/PactTestHelper.cs"
    )
    
    for file in "${files[@]}"; do
        if [ -f "$file" ]; then
            echo "  ✅ $file"
        else
            echo "  ❌ $file (missing)"
            return 1
        fi
    done
    return 0
}

# Function to check dependencies
check_dependencies() {
    echo "📦 Checking dependencies..."
    
    # Check Node.js dependencies
    if [ -f "src/frontend/package.json" ]; then
        if grep -q "@pact-foundation/pact" "src/frontend/package.json"; then
            echo "  ✅ Frontend Pact dependencies"
        else
            echo "  ❌ Frontend Pact dependencies missing"
            return 1
        fi
    fi
    
    # Check Go dependencies
    if [ -f "src/checkout/go.mod" ]; then
        if grep -q "pact-foundation/pact-go" "src/checkout/go.mod"; then
            echo "  ✅ Checkout Pact dependencies"
        else
            echo "  ❌ Checkout Pact dependencies missing"
            return 1
        fi
    fi
    
    # Check Rust dependencies
    if [ -f "src/shipping/Cargo.toml" ]; then
        if grep -q "pact_verifier" "src/shipping/Cargo.toml"; then
            echo "  ✅ Shipping Pact dependencies"
        else
            echo "  ❌ Shipping Pact dependencies missing"
            return 1
        fi
    fi
    
    # Check C# dependencies
    if [ -f "src/accounting/Accounting.csproj" ]; then
        if grep -q "PactNet" "src/accounting/Accounting.csproj"; then
            echo "  ✅ Accounting Pact dependencies"
        else
            echo "  ❌ Accounting Pact dependencies missing"
            return 1
        fi
    fi
    
    return 0
}

# Function to check tools
check_tools() {
    echo "🔧 Checking required tools..."
    
    local tools=(
        "node:Node.js"
        "npm:npm"
        "go:Go"
        "cargo:Rust/Cargo"
        "dotnet:.NET"
    )
    
    for tool_info in "${tools[@]}"; do
        IFS=':' read -r cmd name <<< "$tool_info"
        if command_exists "$cmd"; then
            echo "  ✅ $name ($cmd)"
        else
            echo "  ⚠️  $name ($cmd) - not found but may not be needed for all tests"
        fi
    done
    return 0
}

# Function to test basic script execution
test_scripts() {
    echo "🧪 Testing script execution..."
    
    # Test pact validation script
    if [ -f "pacts/validate-setup.sh" ] && [ -x "pacts/validate-setup.sh" ]; then
        echo "  ✅ pacts/validate-setup.sh is executable"
    else
        echo "  ❌ pacts/validate-setup.sh is not executable"
        return 1
    fi
    
    # Test contract test runner
    if [ -f "pacts/run-contract-tests.sh" ] && [ -x "pacts/run-contract-tests.sh" ]; then
        echo "  ✅ pacts/run-contract-tests.sh is executable"
    else
        echo "  ❌ pacts/run-contract-tests.sh is not executable"
        return 1
    fi
    
    return 0
}

# Run all checks
echo ""
check_directories
dirs_ok=$?

echo ""
check_files
files_ok=$?

echo ""
check_dependencies
deps_ok=$?

echo ""
check_tools
tools_ok=$?

echo ""
test_scripts
scripts_ok=$?

echo ""
echo "======================================"
echo "📊 Validation Summary:"
echo "======================================"

if [ $dirs_ok -eq 0 ]; then
    echo "✅ Directory structure: OK"
else
    echo "❌ Directory structure: FAILED"
fi

if [ $files_ok -eq 0 ]; then
    echo "✅ Required files: OK"
else
    echo "❌ Required files: FAILED"
fi

if [ $deps_ok -eq 0 ]; then
    echo "✅ Dependencies: OK"
else
    echo "❌ Dependencies: FAILED"
fi

if [ $tools_ok -eq 0 ]; then
    echo "✅ Tools: OK"
else
    echo "⚠️  Tools: Some missing (may be OK)"
fi

if [ $scripts_ok -eq 0 ]; then
    echo "✅ Scripts: OK"
else
    echo "❌ Scripts: FAILED"
fi

echo ""
if [ $dirs_ok -eq 0 ] && [ $files_ok -eq 0 ] && [ $deps_ok -eq 0 ] && [ $scripts_ok -eq 0 ]; then
    echo "🎉 Contract testing setup is valid and ready to use!"
    echo ""
    echo "Next steps:"
    echo "1. Run './pacts/validate-setup.sh' for detailed validation"
    echo "2. Run './pacts/run-contract-tests.sh' to execute contract tests"
    echo "3. Check the documentation in docs/ for usage examples"
    exit 0
else
    echo "❌ Contract testing setup has issues that need to be resolved."
    echo ""
    echo "Please fix the failed checks above before proceeding."
    exit 1
fi