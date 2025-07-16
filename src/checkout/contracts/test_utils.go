package contracts

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/pact-foundation/pact-go/v2/consumer"
	"github.com/pact-foundation/pact-go/v2/matchers"
	"github.com/pact-foundation/pact-go/v2/models"
)

// PactTestHelper provides utilities for Pact testing in Go
type PactTestHelper struct {
	PactDir string
}

// NewPactTestHelper creates a new instance with default pact directory
func NewPactTestHelper() *PactTestHelper {
	return &PactTestHelper{
		PactDir: filepath.Join("..", "..", "pacts", "consumer-contracts"),
	}
}

// SetupMessagePact sets up a Pact for message-based testing (Kafka)
func (p *PactTestHelper) SetupMessagePact(consumerName, providerName string) *consumer.V2HTTPMockProvider {
	return &consumer.V2HTTPMockProvider{
		Consumer: consumerName,
		Provider: providerName,
		PactDir:  p.PactDir,
		LogLevel: "INFO",
	}
}

// GetOrderMessageMatchers returns common matchers for order messages
func (p *PactTestHelper) GetOrderMessageMatchers() map[string]interface{} {
	return map[string]interface{}{
		"order_id":             matchers.Like("order-123"),
		"shipping_tracking_id": matchers.Regex("Z[A-Z0-9]{8}", "Z12345678"),
		"shipping_cost":        p.GetMoneyMatchers(),
		"shipping_address":     p.GetAddressMatchers(),
		"items":                matchers.EachLike(p.GetOrderItemMatchers(), 1),
	}
}

// GetMoneyMatchers returns matchers for Money protobuf messages
func (p *PactTestHelper) GetMoneyMatchers() map[string]interface{} {
	return map[string]interface{}{
		"currency_code": matchers.Like("USD"),
		"units":         matchers.Like(int64(8)),
		"nanos":         matchers.Like(int32(990000000)),
	}
}

// GetAddressMatchers returns matchers for Address protobuf messages
func (p *PactTestHelper) GetAddressMatchers() map[string]interface{} {
	return map[string]interface{}{
		"street_address": matchers.Like("1600 Amphitheatre Parkway"),
		"city":           matchers.Like("Mountain View"),
		"state":          matchers.Like("CA"),
		"country":        matchers.Like("United States"),
		"zip_code":       matchers.Regex("^\\d{5}$", "94043"),
	}
}

// GetOrderItemMatchers returns matchers for OrderItem protobuf messages
func (p *PactTestHelper) GetOrderItemMatchers() map[string]interface{} {
	return map[string]interface{}{
		"item": map[string]interface{}{
			"product_id": matchers.Like("OLJCESPC7Z"),
			"quantity":   matchers.Like(int32(2)),
		},
		"cost": p.GetMoneyMatchers(),
	}
}

// ValidatePactFile checks if a pact file exists and has valid structure
func (p *PactTestHelper) ValidatePactFile(filename string) error {
	filePath := filepath.Join(p.PactDir, filename)

	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		return fmt.Errorf("pact file does not exist: %s", filePath)
	}

	data, err := os.ReadFile(filePath)
	if err != nil {
		return fmt.Errorf("failed to read pact file: %w", err)
	}

	var pact models.Pact
	if err := json.Unmarshal(data, &pact); err != nil {
		return fmt.Errorf("invalid pact file format: %w", err)
	}

	if pact.Consumer.Name == "" || pact.Provider.Name == "" {
		return fmt.Errorf("pact file missing consumer or provider name")
	}

	return nil
}

// CleanupPactFiles removes generated pact files (for testing)
func (p *PactTestHelper) CleanupPactFiles() error {
	if _, err := os.Stat(p.PactDir); os.IsNotExist(err) {
		return nil // Directory doesn't exist, nothing to clean
	}

	files, err := filepath.Glob(filepath.Join(p.PactDir, "*.json"))
	if err != nil {
		return fmt.Errorf("failed to list pact files: %w", err)
	}

	for _, file := range files {
		if err := os.Remove(file); err != nil {
			return fmt.Errorf("failed to remove pact file %s: %w", file, err)
		}
	}

	return nil
}

// TestDataFactory provides consistent test data for contract tests
type TestDataFactory struct{}

// GetValidOrderMessage returns a valid order message for testing
func (t *TestDataFactory) GetValidOrderMessage() map[string]interface{} {
	return map[string]interface{}{
		"order_id":             "order-123",
		"shipping_tracking_id": "Z12345678",
		"shipping_cost": map[string]interface{}{
			"currency_code": "USD",
			"units":         int64(8),
			"nanos":         int32(990000000),
		},
		"shipping_address": map[string]interface{}{
			"street_address": "1600 Amphitheatre Parkway",
			"city":           "Mountain View",
			"state":          "CA",
			"country":        "United States",
			"zip_code":       "94043",
		},
		"items": []map[string]interface{}{
			{
				"item": map[string]interface{}{
					"product_id": "OLJCESPC7Z",
					"quantity":   int32(2),
				},
				"cost": map[string]interface{}{
					"currency_code": "USD",
					"units":         int64(15),
					"nanos":         int32(990000000),
				},
			},
		},
	}
}

// GetEmptyOrderMessage returns an order message with empty items for error testing
func (t *TestDataFactory) GetEmptyOrderMessage() map[string]interface{} {
	orderMsg := t.GetValidOrderMessage()
	orderMsg["items"] = []map[string]interface{}{}
	return orderMsg
}

// GetInvalidOrderMessage returns an order message with missing required fields
func (t *TestDataFactory) GetInvalidOrderMessage() map[string]interface{} {
	return map[string]interface{}{
		"order_id": "order-123",
		// Missing required fields for error testing
	}
}
