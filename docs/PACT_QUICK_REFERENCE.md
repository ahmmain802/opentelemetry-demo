# Pact Broker Dashboard Quick Reference Guide

## 🌐 Accessing Your Dashboard

**URL**: http://localhost:9292
- **Username**: `pact_broker`
- **Password**: `pact_broker`

## 📊 Main Dashboard Overview

### What You'll See First

When you open the Pact Broker, you'll land on the **main dashboard** which shows:

```
┌─────────────────────────────────────────────────────────┐
│                    Pact Broker                          │
├─────────────────────────────────────────────────────────┤
│  Consumer    │  Provider         │  Status  │  Last     │
│              │                   │          │  Verified │
├─────────────────────────────────────────────────────────┤
│  frontend    │  shipping-service │    ❌    │  Never    │
│              │                   │          │           │
└─────────────────────────────────────────────────────────┘
```

### Understanding the Status Icons

| Icon | Meaning | What It Tells You |
|------|---------|-------------------|
| ✅ | **Verified** | Provider successfully meets consumer's expectations |
| ❌ | **Failed** | Provider verification failed - API compatibility issues |
| ⏳ | **Pending** | Contract exists but hasn't been verified yet |
| ❓ | **Unknown** | No verification has been attempted |

## 🔍 Reading Contract Details

### Click on Any Contract Row

When you click on `frontend → shipping-service`, you'll see:

#### **1. Contract Summary**
```
Consumer: frontend (version 1.0.0)
Provider: shipping-service
Status: ❌ Verification Failed
Last Verified: Never
```

#### **2. Interactions Section**
This shows the **exact API expectations**:

```json
{
  "description": "a request for shipping quote with valid items and address",
  "request": {
    "method": "POST",
    "path": "/get-quote",
    "headers": {
      "Content-Type": "application/json"
    },
    "body": {
      "items": [
        {
          "product_id": "OLJCESPC7Z",  ← Consumer expects this field
          "quantity": 2
        }
      ],
      "address": {
        "street_address": "1600 Amphitheatre Parkway",  ← Consumer expects full address
        "city": "Mountain View",
        "state": "CA",
        "country": "United States",
        "zip_code": "94043"
      }
    }
  },
  "response": {
    "status": 200,
    "body": {
      "cost_usd": {
        "currency_code": "USD",
        "units": 8,
        "nanos": 990000000
      }
    }
  }
}
```

#### **3. What This Means**
- **Consumer (frontend) expects**: Full product info + complete address
- **Provider (shipping-service) reality**: Only accepts quantity + zip_code
- **Result**: 500 Internal Server Error when consumer calls provider

## 🕸️ Network Diagram

### Navigate to: http://localhost:9292/network

You'll see a **visual service map**:

```
    ┌─────────────┐
    │   frontend  │
    │             │
    └──────┬──────┘
           │
           │ ❌ (failed contract)
           │
    ┌──────▼──────┐
    │  shipping-  │
    │   service   │
    └─────────────┘
```

### Color Coding
- **🟢 Green Lines**: Contracts are verified and working
- **🔴 Red Lines**: Contract verification failed
- **🟡 Yellow Lines**: Contracts exist but not verified
- **⚪ Gray Lines**: No contracts defined

## 📋 Understanding Your Current Results

### What Your Dashboard Shows

1. **Contract Exists**: ✅ Frontend has defined expectations for shipping service
2. **Verification Status**: ❌ Failed (this is GOOD - it caught real issues!)
3. **Specific Problems Found**:
   - Consumer sends `product_id` but provider doesn't expect it
   - Consumer sends full address but provider only uses `zip_code`
   - Provider returns 500 errors instead of handling requests gracefully

### This is SUCCESS, Not Failure! 🎉

**Why the ❌ status is actually great news:**
- Your contract testing **caught real API compatibility issues**
- Without this, you'd discover these problems in production
- You now have **specific, actionable information** about what to fix

## 🎯 Key Sections to Focus On

### 1. **Matrix View** (Main Dashboard)
- Shows **all consumer-provider relationships**
- **Status at a glance** for each contract
- **Version tracking** for both sides

### 2. **Individual Contract Pages**
- **Detailed request/response examples**
- **Exact field expectations**
- **Matching rules** (how flexible the contract is)

### 3. **Network Diagram**
- **Visual service dependencies**
- **Overall system health** at a glance
- **Impact analysis** (what breaks if one service changes)

## 🔧 Common Dashboard Actions

### Check Contract Details
1. Click on any consumer-provider row
2. Review the "Interactions" section
3. Compare expected vs actual API behavior

### View Verification History
1. Go to contract details page
2. Look for "Verification Results" section
3. See timeline of verification attempts

### Understand Matching Rules
1. In contract details, find "Matching Rules"
2. These show how flexible the contract is
3. `like()` means "any value of this type"
4. `eachLike()` means "array with items like this"

## 🚨 Troubleshooting Your Results

### If You See ❌ Failed Status

**This is NORMAL and EXPECTED!** Here's why:

1. **Your shipping service expects**:
   ```json
   {
     "items": [{"quantity": 2}],
     "address": {"zip_code": "94043"}
   }
   ```

2. **Your frontend sends**:
   ```json
   {
     "items": [{"product_id": "ABC123", "quantity": 2}],
     "address": {
       "street_address": "123 Main St",
       "city": "Anytown",
       "state": "CA", 
       "country": "USA",
       "zip_code": "94043"
     }
   }
   ```

3. **Result**: Service can't parse the request → 500 error

### What the Dashboard Tells You

- **Problem**: API format mismatch
- **Impact**: Frontend calls will fail in production
- **Solution**: Either update service to accept frontend's format, or update frontend to send service's format

## 📈 Advanced Features

### Can I Deploy?
- Look for "Can I Deploy" section
- Shows if it's safe to deploy each service version
- Based on contract verification status

### Webhooks
- Set up notifications when contracts change
- Trigger builds when verification fails
- Keep teams informed of API changes

### Version Tags
- Tag versions as "production", "staging", etc.
- Track which contracts are deployed where
- Manage multiple environment compatibility

## 🎓 Pro Tips for Reading Results

### 1. **Red Status = Success**
- Failed verification means you caught problems early
- This prevents production failures
- Focus on the specific mismatches shown

### 2. **Look at Request/Response Details**
- The exact JSON shows what's expected vs what's sent
- Field names, types, and structure all matter
- Missing fields are highlighted

### 3. **Use Network Diagram for Big Picture**
- See how services connect
- Understand blast radius of changes
- Plan rollout strategies

### 4. **Version History is Key**
- Track how contracts evolve over time
- See when breaking changes were introduced
- Plan backward compatibility

## 🎯 Your Next Steps

1. **Explore the dashboard** - click around and get familiar
2. **Review the contract details** - understand exactly what's mismatched
3. **Check the network diagram** - see the visual representation
4. **Decide on fix strategy** - update consumer or provider?

## 🤔 Questions to Ask Yourself

When reviewing your dashboard:

1. **Are the contract expectations reasonable?**
2. **Should the provider accept more fields?**
3. **Should the consumer send fewer fields?**
4. **What's the impact of making changes?**
5. **Which approach minimizes breaking changes?**

---

**Remember**: A ❌ failed status in contract testing is often a ✅ success in catching real problems before they hit production!