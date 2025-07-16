# Pact Contract Testing - Quick Reference

## 🚀 Quick Start Commands

```bash
# Run all contract tests
./pacts/run-contract-tests.sh

# Run only consumer tests (generates contracts)
./pacts/run-contract-tests.sh --consumer-only

# Run only provider verification
./pacts/run-contract-tests.sh --provider-only

# Skip dependency installation
./pacts/run-contract-tests.sh --skip-deps
```

## 📁 Project Structure

```
pacts/
├── consumer-contracts/          # 📄 Generated pact files
├── provider-verification/       # 📋 Verification logs
├── shared/                     # 🔧 Shared utilities
│   ├── pact-config.yml        # ⚙️ Configuration
│   └── test-utils.ts          # 🛠️ TypeScript helpers
└── run-contract-tests.sh      # 🎯 Master script

src/
├── frontend/__tests__/contracts/    # 🌐 Frontend consumer tests
├── checkout/contracts/              # 🛒 Checkout consumer tests  
├── shipping/tests/                  # 🚚 Shipping provider tests
└── accounting/Tests/                # 💰 Accounting provider tests
```

## 🔄 Testing Workflow

```
1. Write Consumer Test → 2. Generate Pact File → 3. Verify Provider → 4. Deploy
   (Frontend)              (contract.json)        (Shipping API)     (✅ Safe)
```

## 📝 Common Matchers

### TypeScript/JavaScript
```typescript
import { MatchersV3 } from '@pact-foundation/pact';
const { like, eachLike, term, regex } = MatchersV3;

// Type matching
like(123)                    // Any number
like('string')               // Any string
like(true)                   // Any boolean

// Array matching
eachLike(like('item'))       // Array of strings
eachLike({id: like(1)}, {min: 2}) // Array with min 2 objects

// Pattern matching
regex('^\\d{5}$', '12345')   // Zip code pattern
term({
  generate: 'hello',
  matcher: '^h.*o$'
})

// Optional fields
like('value').optional()     // Field may be missing
```

### Go
```go
import "github.com/pact-foundation/pact-go/v2/matchers"

matchers.Like("string")           // Any string
matchers.Like(123)               // Any number
matchers.EachLike(obj, 1)        // Array with min 1 item
matchers.Regex("^\\d+$", "123")  // Pattern matching
```

### Rust
```rust
// In pact_utils.rs helper functions
json!({
    "id": like(123),
    "name": like("Product Name"),
    "tags": each_like(like("tag"), 1)
})
```

### C#
```csharp
using PactNet.Matchers;

Match.Type("string")              // Any string
Match.Type(123)                   // Any number
Match.MinType(obj, 1)            // Array with min 1 item
Match.Regex("^\\d+$", "123")     // Pattern matching
```

## 🎯 Test Templates

### Consumer Test Template (TypeScript)
```typescript
describe('Service Contract', () => {
  const provider = PactTestHelper.setupPactProvider('Consumer', 'Provider', 3001);

  it('should handle successful request', async () => {
    await provider
      .given('provider state')
      .uponReceiving('description of request')
      .withRequest({
        method: 'POST',
        path: '/api/endpoint',
        headers: { 'Content-Type': 'application/json' },
        body: { /* request body */ }
      })
      .willRespondWith({
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: { /* expected response */ }
      });

    await provider.executeTest(async (mockService) => {
      // Your actual API call here
      const response = await fetch(`${mockService.url}/api/endpoint`);
      // Assertions
    });
  });
});
```

### Provider Verification Template (Rust)
```rust
#[tokio::test]
async fn verify_consumer_contract() {
    let helper = PactTestHelper::new();
    let pact_files = helper.load_pact_files("consumer", "provider").unwrap();
    let provider = helper.setup_provider_verification("Provider", "http://localhost:8080");
    
    let result = verify_provider(provider, pact_files, &HashMap::new()).await;
    assert!(result.is_ok());
}
```

## 🐛 Common Issues & Fixes

| Problem | Solution |
|---------|----------|
| **No pact file generated** | Check test passes and pact directory exists |
| **Provider verification fails** | Ensure provider is running and accessible |
| **Matcher errors** | Use `like()` instead of exact values |
| **Timeout issues** | Increase timeout in pact configuration |
| **State setup fails** | Verify provider state names match exactly |

## 🔍 Debug Commands

```bash
# Check pact files
ls -la pacts/consumer-contracts/
cat pacts/consumer-contracts/*.json | jq .

# Validate JSON structure
find pacts -name "*.json" -exec jq . {} \;

# Check service health
curl http://localhost:8080/health

# View logs
cat pacts/provider-verification/*.log

# Test individual services
cd src/frontend && npm run test:pact:consumer
cd src/checkout && go test -v ./contracts/...
cd src/shipping && cargo test --test pact_verification
cd src/accounting && dotnet test --filter PactVerification
```

## 📊 Best Practices Checklist

### ✅ Consumer Tests
- [ ] Use descriptive test names
- [ ] Focus on behavior, not implementation
- [ ] Use appropriate matchers (`like()`, `eachLike()`)
- [ ] Test error scenarios (4xx, 5xx responses)
- [ ] Keep contracts minimal and focused

### ✅ Provider Verification
- [ ] Set up proper provider states
- [ ] Clean up test data after each test
- [ ] Verify against real service endpoints
- [ ] Handle authentication in tests
- [ ] Run verification in CI pipeline

### ✅ Contract Design
- [ ] Version your contracts
- [ ] Make new fields optional initially
- [ ] Use semantic versioning
- [ ] Document breaking changes
- [ ] Maintain backward compatibility

## 🚨 Anti-Patterns to Avoid

❌ **Don't** use exact values everywhere
```typescript
// Bad
body: { id: 123, name: "Exact Name" }

// Good  
body: { id: like(123), name: like("Any Name") }
```

❌ **Don't** test implementation details
```typescript
// Bad - testing internal structure
body: { 
  internal_id: like(123),
  debug_info: like("internal details")
}

// Good - testing public interface
body: {
  user_id: like(123),
  display_name: like("User Name")
}
```

❌ **Don't** create brittle contracts
```typescript
// Bad - too specific
body: {
  created_at: "2024-01-15T10:30:00.000Z",
  items: ["item1", "item2", "item3"]
}

// Good - flexible
body: {
  created_at: regex(ISO_DATE_PATTERN, "2024-01-15T10:30:00.000Z"),
  items: eachLike(like("item"), {min: 1})
}
```

## 🔗 Useful Links

- [Full Documentation](./CONTRACT_TESTING_GUIDE.md)
- [Pact Official Docs](https://docs.pact.io/)
- [Matcher Reference](https://docs.pact.io/implementation_guides/javascript/docs/matching)
- [Best Practices](https://docs.pact.io/best_practices/contract_tests_not_functional_tests)

## 💡 Pro Tips

1. **Start Simple**: Begin with one happy path, add complexity gradually
2. **Test Locally**: Always run tests locally before pushing
3. **Use Helpers**: Leverage shared utilities for consistent patterns
4. **Document States**: Keep provider states well-documented
5. **Monitor CI**: Set up alerts for contract test failures
6. **Team Communication**: Share contract changes with affected teams

---

*Need help? Check the [full guide](./CONTRACT_TESTING_GUIDE.md) or ask in #contract-testing Slack channel*