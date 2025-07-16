import { PactV3, MatchersV3 } from '@pact-foundation/pact';
import * as path from 'path';
import * as fs from 'fs';

const { like, eachLike, term, regex } = MatchersV3;

/**
 * Shared Pact test utilities for TypeScript services
 */
export class PactTestHelper {
  /**
   * Set up a Pact provider for consumer testing
   */
  static setupPactProvider(consumerName: string, providerName: string, port: number): PactV3 {
    return new PactV3({
      consumer: consumerName,
      provider: providerName,
      port: port,
      dir: path.resolve(__dirname, '../consumer-contracts'),
      logLevel: 'INFO',
      spec: 3,
    });
  }

  /**
   * Clean up generated pact files (for testing)
   */
  static cleanupPactFiles(): void {
    const pactDir = path.resolve(__dirname, '../consumer-contracts');
    if (fs.existsSync(pactDir)) {
      const files = fs.readdirSync(pactDir);
      files.forEach(file => {
        if (file.endsWith('.json')) {
          fs.unlinkSync(path.join(pactDir, file));
        }
      });
    }
  }

  /**
   * Validate that a pact file exists and has valid structure
   */
  static validatePactFile(filePath: string): boolean {
    try {
      if (!fs.existsSync(filePath)) {
        return false;
      }
      
      const pactContent = JSON.parse(fs.readFileSync(filePath, 'utf8'));
      
      // Basic validation of pact file structure
      return !!(
        pactContent.consumer &&
        pactContent.provider &&
        pactContent.interactions &&
        Array.isArray(pactContent.interactions)
      );
    } catch (error) {
      console.error('Error validating pact file:', error);
      return false;
    }
  }

  /**
   * Common matchers for shipping service responses
   */
  static getShippingMatchers() {
    return {
      costUsd: like({
        currency_code: 'USD',
        units: like(8),
        nanos: like(990000000)
      }),
      
      trackingId: regex({
        generate: 'Z12345678',
        matcher: '^Z[A-Z0-9]{8}$'
      })
    };
  }

  /**
   * Common matchers for address objects
   */
  static getAddressMatchers() {
    return {
      street_address: like('1600 Amphitheatre Parkway'),
      city: like('Mountain View'),
      state: like('CA'),
      country: like('United States'),
      zip_code: regex({
        generate: '94043',
        matcher: '^\\d{5}$'
      })
    };
  }

  /**
   * Common matchers for cart items
   */
  static getCartItemMatchers() {
    return eachLike({
      product_id: like('OLJCESPC7Z'),
      quantity: like(2)
    }, { min: 1 });
  }

  /**
   * Common error response matchers
   */
  static getErrorMatchers() {
    return {
      badRequest: {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
        body: like({
          error: like('Bad Request'),
          message: like('Invalid request format')
        })
      },
      
      internalError: {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
        body: like({
          error: like('Internal Server Error'),
          message: like('Service temporarily unavailable')
        })
      }
    };
  }
}

/**
 * Test data factory for consistent test data across services
 */
export class TestDataFactory {
  static getValidShippingRequest() {
    return {
      items: [
        { product_id: 'OLJCESPC7Z', quantity: 2 },
        { product_id: '66VCHSJNUP', quantity: 1 }
      ],
      address: {
        street_address: '1600 Amphitheatre Parkway',
        city: 'Mountain View',
        state: 'CA',
        country: 'United States',
        zip_code: '94043'
      }
    };
  }

  static getValidShippingResponse() {
    return {
      cost_usd: {
        currency_code: 'USD',
        units: 8,
        nanos: 990000000
      }
    };
  }

  static getEmptyCartRequest() {
    return {
      items: [],
      address: {
        street_address: '1600 Amphitheatre Parkway',
        city: 'Mountain View',
        state: 'CA',
        country: 'United States',
        zip_code: '94043'
      }
    };
  }

  static getInvalidAddressRequest() {
    return {
      items: [
        { product_id: 'OLJCESPC7Z', quantity: 1 }
      ],
      address: {
        street_address: '',
        city: '',
        state: '',
        country: '',
        zip_code: ''
      }
    };
  }
}

export { like, eachLike, term, regex };