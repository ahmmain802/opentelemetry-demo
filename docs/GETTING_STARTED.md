# Getting Started with Contract Testing

Welcome to contract testing! This guide will get you up and running quickly, even if you're new to the concept.

## 🎯 What You'll Learn

By the end of this guide, you'll be able to:
- ✅ Understand what contract testing is and why it's useful
- ✅ Set up your development environment
- ✅ Write your first consumer contract test
- ✅ Implement provider verification
- ✅ Run the complete contract testing workflow

## 📚 Documentation Overview

We've created comprehensive documentation to support developers at all levels:

| Document | Purpose | Best For |
|----------|---------|----------|
| **[🚀 This Guide](GETTING_STARTED.md)** | Quick start and overview | First-time users |
| **[📖 Complete Guide](CONTRACT_TESTING_GUIDE.md)** | In-depth tutorial with examples | Learning the concepts |
| **[⚡ Quick Reference](PACT_QUICK_REFERENCE.md)** | Commands and templates | Daily development |
| **[🎯 Example Walkthrough](EXAMPLE_WALKTHROUGH.md)** | Step-by-step implementation | Hands-on learning |

## 🏃‍♂️ Quick Start (5 Minutes)

### Step 1: Validate Your Setup
```bash
# Check if your environment is ready
./pacts/validate-setup.sh
```

If you see ✅ for most items, you're good to go! If not, follow the installation instructions provided.

### Step 2: Run the Example
```bash
# Run all contract tests
./pacts/run-contract-tests.sh --help

# See what's available
ls -la pacts/
ls -la docs/
```

### Step 3: Explore the Documentation
```bash
# Read the comprehensive guide
open docs/CONTRACT_TESTING_GUIDE.md

# Or start with the example
open docs/EXAMPLE_WALKTHROUGH.md
```

## 🎓 Learning Path for Junior Developers

### Phase 1: Understanding (30 minutes)
1. **Read the concept overview** in [CONTRACT_TESTING_GUIDE.md](CONTRACT_TESTING_GUIDE.md#what-is-contract-testing)
2. **Understand the workflow** - Consumer tests → Pact files → Provider verification
3. **Review the project structure** - See where files are organized

### Phase 2: Hands-On Practice (1 hour)
1. **Follow the complete example** in [EXAMPLE_WALKTHROUGH.md](EXAMPLE_WALKTHROUGH.md)
2. **Run the validation script** to check your setup
3. **Experiment with the test scripts** using different options

### Phase 3: Implementation (2-3 hours)
1. **Write your first consumer test** using the templates
2. **Implement provider verification** for your service
3. **Run the complete workflow** and debug any issues

### Phase 4: Mastery (Ongoing)
1. **Study the best practices** section
2. **Explore advanced patterns** for your use cases
3. **Contribute improvements** to the shared utilities

## 🛠️ Development Environment Setup

### Prerequisites Checklist
- [ ] Node.js 18+ (for frontend tests)
- [ ] Go 1.21+ (for checkout service tests)
- [ ] Rust 1.70+ (for shipping service tests)
- [ ] .NET 8.0+ (for accounting service tests)
- [ ] jq (optional, for debugging JSON)

### Quick Installation (macOS)
```bash
# Install Node.js
brew install node

# Install Go
brew install go

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install .NET
brew install --cask dotnet

# Install jq (optional)
brew install jq
```

### Validation
```bash
# Validate your setup
./pacts/validate-setup.sh

# Should show mostly ✅ green checkmarks
```

## 🎯 Your First Contract Test

Let's create a simple contract test to get you started:

### 1. Consumer Test (Frontend)
```typescript
// src/frontend/__tests__/contracts/my-first-test.ts
import { PactV3, MatchersV3 } from '@pact-foundation/pact';

const { like } = MatchersV3;

describe('My First Contract Test', () => {
  const provider = new PactV3({
    consumer: 'Frontend',
    provider: 'MyService',
    port: 3001,
    dir: '../../../pacts/consumer-contracts'
  });

  it('should get user data', async () => {
    await provider
      .given('user exists')
      .uponReceiving('a request for user data')
      .withRequest({
        method: 'GET',
        path: '/users/123'
      })
      .willRespondWith({
        status: 200,
        body: {
          id: like(123),
          name: like('John Doe'),
          email: like('john@example.com')
        }
      });

    await provider.executeTest(async (mockService) => {
      const response = await fetch(`${mockService.url}/users/123`);
      const user = await response.json();
      
      expect(response.status).toBe(200);
      expect(user.id).toBe(123);
      expect(user.name).toBe('John Doe');
    });
  });
});
```

### 2. Run the Test
```bash
cd src/frontend
npm run test:pact:consumer
```

### 3. Check the Generated Contract
```bash
cat pacts/consumer-contracts/frontend-myservice.json | jq .
```

## 🔧 Common Commands

```bash
# Validate your setup
./pacts/validate-setup.sh

# Run all contract tests
./pacts/run-contract-tests.sh

# Run only consumer tests (generates contracts)
./pacts/run-contract-tests.sh --consumer-only

# Run only provider verification
./pacts/run-contract-tests.sh --provider-only

# Get help
./pacts/run-contract-tests.sh --help

# Debug pact files
find pacts/consumer-contracts -name "*.json" -exec jq . {} \;

# Check logs
cat pacts/provider-verification/*.log
```

## 🐛 Troubleshooting

### Common Issues for Beginners

#### "Command not found" errors
```bash
# Make sure scripts are executable
chmod +x pacts/*.sh

# Check your PATH includes the tools
which node go cargo dotnet
```

#### "Pact file not generated"
```bash
# Check the test actually runs
cd src/frontend
npm test -- --verbose

# Verify directory exists
ls -la pacts/consumer-contracts/
```

#### "Provider verification fails"
```bash
# Make sure the service is running
curl http://localhost:8080/health

# Check the pact file is valid
cat pacts/consumer-contracts/*.json | jq .
```

#### "Tests are flaky"
- Use `like()` matchers instead of exact values
- Don't test implementation details
- Focus on the contract, not the data

### Getting Help

1. **Check the documentation** - Most answers are in the guides
2. **Run the validation script** - `./pacts/validate-setup.sh`
3. **Look at the examples** - Real working code in the walkthrough
4. **Ask for help** - Your team is here to support you!

## 🎉 Success Criteria

You'll know you're successful when:

✅ **Setup validation passes** - All green checkmarks  
✅ **Consumer tests run** and generate pact files  
✅ **Provider verification passes** against contracts  
✅ **You understand the workflow** - Consumer → Contract → Provider  
✅ **Tests run fast** - Under 30 seconds for the full suite  

## 🚀 Next Steps

Once you're comfortable with the basics:

1. **Read the advanced sections** in the complete guide
2. **Explore different patterns** - HTTP, Kafka, authentication
3. **Contribute to shared utilities** - Help improve the tooling
4. **Teach others** - Share your knowledge with the team

## 📖 Additional Resources

- [Pact Official Documentation](https://docs.pact.io/)
- [Contract Testing Best Practices](https://pactflow.io/blog/contract-testing-best-practices/)
- [Microservices Testing Pyramid](https://martinfowler.com/articles/microservice-testing/)
- [Consumer-Driven Contracts](https://martinfowler.com/articles/consumerDrivenContracts.html)

## 💡 Pro Tips for Junior Developers

1. **Start simple** - One happy path test first
2. **Use the templates** - Don't write everything from scratch
3. **Test locally first** - Before pushing to CI
4. **Read error messages carefully** - They usually tell you what's wrong
5. **Ask questions early** - Don't struggle alone
6. **Practice regularly** - Contract testing becomes intuitive with practice

---

**Ready to dive deeper?** Check out the [Complete Guide](CONTRACT_TESTING_GUIDE.md) for comprehensive examples and best practices!

**Need a quick reference?** Use the [Quick Reference](PACT_QUICK_REFERENCE.md) for commands and templates.

**Want to see it in action?** Follow the [Example Walkthrough](EXAMPLE_WALKTHROUGH.md) step by step.