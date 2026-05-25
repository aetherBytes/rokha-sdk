import type { RokhaClient } from './client.js';
import type { MarketplaceListing, MarketplaceSearchRequest } from './types.js';

export class MarketplaceClient {
  constructor(private client: RokhaClient) {}

  async health(): Promise<{ status: string }> {
    return this.client.get('/api/crossroads/health');
  }

  async list(): Promise<MarketplaceListing[]> {
    return this.client.get('/api/marketplace/listings');
  }

  async get(id: string): Promise<MarketplaceListing> {
    return this.client.get(`/api/marketplace/listings/${id}`);
  }

  async create(listing: Partial<MarketplaceListing>): Promise<MarketplaceListing> {
    return this.client.post('/api/marketplace/listings', listing);
  }

  async search(request: MarketplaceSearchRequest): Promise<MarketplaceListing[]> {
    return this.client.post('/api/marketplace/search', request);
  }

  async featured(): Promise<MarketplaceListing[]> {
    return this.client.get('/api/marketplace/featured');
  }

  async stats(): Promise<unknown> {
    return this.client.get('/api/marketplace/stats');
  }

  // --- Discovery ---

  async discoverTools(): Promise<unknown> {
    return this.client.get('/api/discovery/tools');
  }

  async discoverAgents(): Promise<unknown> {
    return this.client.get('/api/discovery/agents');
  }

  async discoverAll(): Promise<unknown> {
    return this.client.get('/api/discovery/all');
  }
}
