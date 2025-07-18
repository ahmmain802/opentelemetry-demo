import { Pact, Matchers } from '@pact-foundation/pact';
import path from 'path';
import { Address, CartItem } from '../../protos/demo';

const { like, eachLike, term } = Matchers;

describe('Shipping Service Consumer Contract Tests', () => {
  const provider = new Pact({
    consumer: 'frontend',
    provider: 'shipping-service',
    port: 1234,
    log: path.resolve(process.cwd(), 'logs', 'pact.log'),
    dir: path.resolve(process.cwd(), '../../pacts/consumer-contracts'),
    logLevel: 'info',
  });

  beforeAll(() => provider.setup());
  afterEach(() => provider.verify());
  afterAll(() => provider.finalize());

  const validCartItems: CartItem[] = [
    {
      productId: 'OLJCESPC7Z',
      quantity: 2,
    },
    {
      productId: '66VCHSJNUP', 
      quantity: 1,
    },
  ];

  const validAddress: Address = {
    streetAddress: '1600 Amphitheatre Parkway',
    city: 'Mountain View',
    state: 'CA',
    country: 'United States',
    zipCode: '94043',
  };

  describe('GET shipping quote', () => {

    it('should return shipping cost for valid cart items and address', async () => {
      // Define the expected interaction
      await provider.addInteraction({
        state: 'shipping service is available',
        uponReceiving: 'a request for shipping quote with valid items and address',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(2),
            }),
            address: like({
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: 'CA',
              country: 'United States',
              zip_code: '94043',
            }),
          },
        },
        willRespondWith: {
          status: 200,
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            cost_usd: {
              currency_code: 'USD',
              units: like(8),
              nanos: like(990000000),
            },
          },
        },
      });

      // Set up environment variable for the test
      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        // Import ShippingGateway after setting environment variable
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        // Execute the actual request
        const response = await ShippingGateway.getShippingCost(validCartItems, validAddress);

        // Verify the response structure
        expect(response).toBeDefined();
        expect(response.costUsd).toBeDefined();
        expect(response.costUsd?.currencyCode).toBe('USD');
        expect(typeof response.costUsd?.units).toBe('number');
        expect(typeof response.costUsd?.nanos).toBe('number');
        expect(response.costUsd?.units).toBeGreaterThanOrEqual(0);
        expect(response.costUsd?.nanos).toBeGreaterThanOrEqual(0);
        expect(response.costUsd?.nanos).toBeLessThan(1000000000);
      } finally {
        // Restore original environment variable
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });

    it('should handle single item requests', async () => {
      const singleItem: CartItem[] = [
        {
          productId: 'OLJCESPC7Z',
          quantity: 1,
        },
      ];

      await provider.addInteraction({
        state: 'shipping service is available',
        uponReceiving: 'a request for shipping quote with single item',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: [
              {
                product_id: like('OLJCESPC7Z'),
                quantity: like(1),
              },
            ],
            address: like({
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: 'CA',
              country: 'United States',
              zip_code: '94043',
            }),
          },
        },
        willRespondWith: {
          status: 200,
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            cost_usd: {
              currency_code: 'USD',
              units: like(8),
              nanos: like(990000000),
            },
          },
        },
      });

      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        // Import ShippingGateway after setting environment variable
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        const response = await ShippingGateway.getShippingCost(singleItem, validAddress);

        expect(response).toBeDefined();
        expect(response.costUsd).toBeDefined();
        expect(response.costUsd?.currencyCode).toBe('USD');
        expect(typeof response.costUsd?.units).toBe('number');
        expect(typeof response.costUsd?.nanos).toBe('number');
      } finally {
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });

    it('should handle international addresses', async () => {
      const internationalAddress: Address = {
        streetAddress: '123 International St',
        city: 'Toronto',
        state: 'ON',
        country: 'Canada',
        zipCode: 'M5V 3A8',
      };

      await provider.addInteraction({
        state: 'shipping service is available',
        uponReceiving: 'a request for shipping quote with international address',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(1),
            }),
            address: like({
              street_address: '123 International St',
              city: 'Toronto',
              state: 'ON',
              country: 'Canada',
              zip_code: 'M5V 3A8',
            }),
          },
        },
        willRespondWith: {
          status: 200,
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            cost_usd: {
              currency_code: 'USD',
              units: like(12),
              nanos: like(990000000),
            },
          },
        },
      });

      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        // Import ShippingGateway after setting environment variable
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        const response = await ShippingGateway.getShippingCost(validCartItems, internationalAddress);

        expect(response).toBeDefined();
        expect(response.costUsd).toBeDefined();
        expect(response.costUsd?.currencyCode).toBe('USD');
        expect(typeof response.costUsd?.units).toBe('number');
        expect(typeof response.costUsd?.nanos).toBe('number');
      } finally {
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });
  });

  describe('Error scenarios', () => {
    it('should return 400 error for empty items array', async () => {
      await provider.addInteraction({
        state: 'shipping service is available',
        uponReceiving: 'a request for shipping quote with empty items array',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: [],
            address: like({
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: 'CA',
              country: 'United States',
              zip_code: '94043',
            }),
          },
        },
        willRespondWith: {
          status: 400,
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            error: {
              code: like('INVALID_REQUEST'),
              message: like('Items array cannot be empty'),
            },
          },
        },
      });

      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        // This should throw an error
        await expect(
          ShippingGateway.getShippingCost([], validAddress)
        ).rejects.toThrow(/HTTP error: 400/);
      } finally {
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });

    it('should return 400 error for missing address fields', async () => {
      const incompleteAddress: Address = {
        streetAddress: '1600 Amphitheatre Parkway',
        city: 'Mountain View',
        state: '', // Missing state
        country: 'United States',
        zipCode: '', // Missing zip code
      };

      await provider.addInteraction({
        state: 'shipping service is available',
        uponReceiving: 'a request for shipping quote with missing address fields',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(1),
            }),
            address: {
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: '',
              country: 'United States',
              zip_code: '',
            },
          },
        },
        willRespondWith: {
          status: 400,
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            error: {
              code: like('INVALID_REQUEST'),
              message: like('Missing required address fields'),
            },
          },
        },
      });

      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        // This should throw an error
        await expect(
          ShippingGateway.getShippingCost(validCartItems, incompleteAddress)
        ).rejects.toThrow(/HTTP error: 400/);
      } finally {
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });

    it('should handle malformed JSON response', async () => {
      await provider.addInteraction({
        state: 'shipping service returns malformed response',
        uponReceiving: 'a request for shipping quote that returns malformed JSON',
        withRequest: {
          method: 'POST',
          path: '/get-quote',
          headers: {
            'Content-Type': 'application/json',
          },
          body: {
            items: eachLike({
              product_id: like('OLJCESPC7Z'),
              quantity: like(1),
            }),
            address: like({
              street_address: '1600 Amphitheatre Parkway',
              city: 'Mountain View',
              state: 'CA',
              country: 'United States',
              zip_code: '94043',
            }),
          },
        },
        willRespondWith: {
          status: 500,
          headers: {
            'Content-Type': 'text/plain',
          },
          body: 'Internal Server Error',
        },
      });

      const originalShippingAddr = process.env.SHIPPING_ADDR;
      process.env.SHIPPING_ADDR = `http://localhost:${provider.opts.port}`;

      try {
        const ShippingGateway = (await import('../../gateways/http/Shipping.gateway')).default;
        
        // This should throw an error
        await expect(
          ShippingGateway.getShippingCost(validCartItems, validAddress)
        ).rejects.toThrow(/HTTP error: 500/);
      } finally {
        if (originalShippingAddr) {
          process.env.SHIPPING_ADDR = originalShippingAddr;
        } else {
          delete process.env.SHIPPING_ADDR;
        }
      }
    });
  });
});