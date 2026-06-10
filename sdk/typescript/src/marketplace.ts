import type { RokhaClient } from './client.js';
import type { MarketplaceListing, MarketplaceSearchRequest } from './types.js';

export class MarketplaceClient {
  constructor(private client: RokhaClient) {}

  async health(): Promise<{ status: string }> {
    return this.client.get('/api/registry/health');
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

  /** Recent mesh activity (cross-registry events, newest first). No auth. */
  async discoverRecent(limit = 20): Promise<unknown> {
    return this.client.get(`/api/discovery/recent?limit=${limit}`);
  }

  /**
   * Stateless one-shot outbound MCP probe/call via the SSRF-guarded proxy.
   * `tools/list` is public; `tools/call` requires the client's bearer JWT.
   */
  async mcpProxy(
    endpoint: string,
    method: 'tools/list' | 'tools/call',
    params: Record<string, unknown> = {},
  ): Promise<unknown> {
    return this.client.post('/api/marketplace/mcp-proxy', { endpoint, method, params });
  }
}
