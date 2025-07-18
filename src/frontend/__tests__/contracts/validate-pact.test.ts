import fs from 'fs';
import path from 'path';

describe('Pact File Validation', () => {
  const pactFilePath = path.resolve(__dirname, '../../../../pacts/consumer-contracts/frontend-shipping-service.json');
  
  it('should have a valid pact file structure', () => {
    // Check if pact file exists
    expect(fs.existsSync(pactFilePath)).toBe(true);
    
    // Read and parse the pact file
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    // Validate basic structure
    expect(pact).toHaveProperty('consumer');
    expect(pact).toHaveProperty('provider');
    expect(pact).toHaveProperty('interactions');
    expect(pact).toHaveProperty('metadata');
    
    // Validate consumer and provider names
    expect(pact.consumer.name).toBe('frontend');
    expect(pact.provider.name).toBe('shipping-service');
    
    // Validate interactions exist
    expect(Array.isArray(pact.interactions)).toBe(true);
    expect(pact.interactions.length).toBeGreaterThan(0);
  });
  
  it('should contain expected happy path interactions', () => {
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    const happyPathInteractions = pact.interactions.filter((interaction: any) => 
      interaction.response.status === 200
    );
    
    expect(happyPathInteractions.length).toBeGreaterThanOrEqual(3);
    
    // Check that all happy path interactions have the expected response structure
    happyPathInteractions.forEach((interaction: any) => {
      expect(interaction.response.body).toHaveProperty('cost_usd');
      expect(interaction.response.body.cost_usd).toHaveProperty('currency_code', 'USD');
      expect(interaction.response.body.cost_usd).toHaveProperty('units');
      expect(interaction.response.body.cost_usd).toHaveProperty('nanos');
    });
  });
  
  it('should contain expected error scenario interactions', () => {
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    const errorInteractions = pact.interactions.filter((interaction: any) => 
      interaction.response.status >= 400
    );
    
    expect(errorInteractions.length).toBeGreaterThanOrEqual(3);
    
    // Check for 400 error interactions
    const badRequestInteractions = errorInteractions.filter((interaction: any) => 
      interaction.response.status === 400
    );
    expect(badRequestInteractions.length).toBeGreaterThanOrEqual(2);
    
    // Check for 500 error interaction
    const serverErrorInteractions = errorInteractions.filter((interaction: any) => 
      interaction.response.status === 500
    );
    expect(serverErrorInteractions.length).toBeGreaterThanOrEqual(1);
  });
  
  it('should have proper request structure for all interactions', () => {
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    pact.interactions.forEach((interaction: any) => {
      // All interactions should be POST requests to /get-quote
      expect(interaction.request.method).toBe('POST');
      expect(interaction.request.path).toBe('/get-quote');
      
      // All interactions should have proper headers
      expect(interaction.request.headers).toHaveProperty('Content-Type', 'application/json');
      
      // All interactions should have a body with items and address
      expect(interaction.request.body).toHaveProperty('items');
      expect(Array.isArray(interaction.request.body.items)).toBe(true);
      
      // Address should be present (even if empty for error scenarios)
      expect(interaction.request.body).toHaveProperty('address');
    });
  });
  
  it('should have proper matching rules', () => {
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    pact.interactions.forEach((interaction: any) => {
      // Check that matching rules exist for flexible contract validation
      if (interaction.request.matchingRules) {
        // Verify that matching rules are properly structured
        Object.keys(interaction.request.matchingRules).forEach(key => {
          expect(interaction.request.matchingRules[key]).toHaveProperty('match');
        });
      }
      
      if (interaction.response.matchingRules) {
        Object.keys(interaction.response.matchingRules).forEach(key => {
          expect(interaction.response.matchingRules[key]).toHaveProperty('match');
        });
      }
    });
  });
  
  it('should have valid metadata', () => {
    const pactContent = fs.readFileSync(pactFilePath, 'utf8');
    const pact = JSON.parse(pactContent);
    
    expect(pact.metadata).toHaveProperty('pact-js');
    expect(pact.metadata).toHaveProperty('pactRust');
    expect(pact.metadata).toHaveProperty('pactSpecification');
    
    // Verify version information is present
    expect(pact.metadata['pact-js']).toHaveProperty('version');
    expect(pact.metadata.pactRust).toHaveProperty('ffi');
    expect(pact.metadata.pactRust).toHaveProperty('models');
    expect(pact.metadata.pactSpecification).toHaveProperty('version');
  });
});