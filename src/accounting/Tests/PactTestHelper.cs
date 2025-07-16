using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using PactNet;
using PactNet.Matchers;

namespace Accounting.Tests
{
    /// <summary>
    /// Shared utilities for Pact testing in C#
    /// </summary>
    public class PactTestHelper
    {
        public string PactDir { get; }
        public string LogDir { get; }

        public PactTestHelper()
        {
            PactDir = Path.Combine("..", "..", "pacts", "consumer-contracts");
            LogDir = Path.Combine("..", "..", "pacts", "provider-verification");
        }

        /// <summary>
        /// Set up Pact verifier for message-based testing (Kafka)
        /// </summary>
        public IPactVerifier SetupMessageVerifier(string providerName)
        {
            var config = new PactVerifierConfig
            {
                LogLevel = PactLogLevel.Information,
                Outputters = new List<IOutput>
                {
                    new ConsoleOutput(),
                    new FileOutput(new FileInfo(Path.Combine(LogDir, $"{providerName}-verification.log")))
                }
            };

            return new PactVerifier(config);
        }

        /// <summary>
        /// Get common matchers for order messages
        /// </summary>
        public Dictionary<string, object> GetOrderMessageMatchers()
        {
            return new Dictionary<string, object>
            {
                ["order_id"] = Match.Type("order-123"),
                ["shipping_tracking_id"] = Match.Regex("Z[A-Z0-9]{8}", "Z12345678"),
                ["shipping_cost"] = GetMoneyMatchers(),
                ["shipping_address"] = GetAddressMatchers(),
                ["items"] = Match.MinType(GetOrderItemMatchers(), 1)
            };
        }

        /// <summary>
        /// Get matchers for Money protobuf messages
        /// </summary>
        public Dictionary<string, object> GetMoneyMatchers()
        {
            return new Dictionary<string, object>
            {
                ["currency_code"] = Match.Type("USD"),
                ["units"] = Match.Type(8L),
                ["nanos"] = Match.Type(990000000)
            };
        }

        /// <summary>
        /// Get matchers for Address protobuf messages
        /// </summary>
        public Dictionary<string, object> GetAddressMatchers()
        {
            return new Dictionary<string, object>
            {
                ["street_address"] = Match.Type("1600 Amphitheatre Parkway"),
                ["city"] = Match.Type("Mountain View"),
                ["state"] = Match.Type("CA"),
                ["country"] = Match.Type("United States"),
                ["zip_code"] = Match.Regex("^\\d{5}$", "94043")
            };
        }

        /// <summary>
        /// Get matchers for OrderItem protobuf messages
        /// </summary>
        public Dictionary<string, object> GetOrderItemMatchers()
        {
            return new Dictionary<string, object>
            {
                ["item"] = new Dictionary<string, object>
                {
                    ["product_id"] = Match.Type("OLJCESPC7Z"),
                    ["quantity"] = Match.Type(2)
                },
                ["cost"] = GetMoneyMatchers()
            };
        }

        /// <summary>
        /// Validate that a pact file exists and has valid structure
        /// </summary>
        public bool ValidatePactFile(string filename)
        {
            try
            {
                var filePath = Path.Combine(PactDir, filename);
                
                if (!File.Exists(filePath))
                {
                    return false;
                }

                var content = File.ReadAllText(filePath);
                var pact = JsonSerializer.Deserialize<JsonElement>(content);

                // Basic validation of pact file structure
                return pact.TryGetProperty("consumer", out _) &&
                       pact.TryGetProperty("provider", out _) &&
                       pact.TryGetProperty("messages", out var messages) &&
                       messages.ValueKind == JsonValueKind.Array;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error validating pact file: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Clean up generated pact files (for testing)
        /// </summary>
        public void CleanupPactFiles()
        {
            if (!Directory.Exists(PactDir))
            {
                return;
            }

            var files = Directory.GetFiles(PactDir, "*.json");
            foreach (var file in files)
            {
                try
                {
                    File.Delete(file);
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"Failed to delete pact file {file}: {ex.Message}");
                }
            }
        }
    }

    /// <summary>
    /// Test data factory for consistent test data across C# tests
    /// </summary>
    public static class TestDataFactory
    {
        /// <summary>
        /// Get valid order message for testing
        /// </summary>
        public static Dictionary<string, object> GetValidOrderMessage()
        {
            return new Dictionary<string, object>
            {
                ["order_id"] = "order-123",
                ["shipping_tracking_id"] = "Z12345678",
                ["shipping_cost"] = new Dictionary<string, object>
                {
                    ["currency_code"] = "USD",
                    ["units"] = 8L,
                    ["nanos"] = 990000000
                },
                ["shipping_address"] = new Dictionary<string, object>
                {
                    ["street_address"] = "1600 Amphitheatre Parkway",
                    ["city"] = "Mountain View",
                    ["state"] = "CA",
                    ["country"] = "United States",
                    ["zip_code"] = "94043"
                },
                ["items"] = new List<Dictionary<string, object>>
                {
                    new Dictionary<string, object>
                    {
                        ["item"] = new Dictionary<string, object>
                        {
                            ["product_id"] = "OLJCESPC7Z",
                            ["quantity"] = 2
                        },
                        ["cost"] = new Dictionary<string, object>
                        {
                            ["currency_code"] = "USD",
                            ["units"] = 15L,
                            ["nanos"] = 990000000
                        }
                    }
                }
            };
        }

        /// <summary>
        /// Get order message with empty items for error testing
        /// </summary>
        public static Dictionary<string, object> GetEmptyOrderMessage()
        {
            var orderMsg = GetValidOrderMessage();
            orderMsg["items"] = new List<Dictionary<string, object>>();
            return orderMsg;
        }

        /// <summary>
        /// Get order message with missing required fields
        /// </summary>
        public static Dictionary<string, object> GetInvalidOrderMessage()
        {
            return new Dictionary<string, object>
            {
                ["order_id"] = "order-123"
                // Missing required fields for error testing
            };
        }
    }
}