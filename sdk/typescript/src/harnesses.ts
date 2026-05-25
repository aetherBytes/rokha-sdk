import type { RokhaClient } from './client.js';
import type { Harness, HarnessCreateRequest, HarnessUpdateRequest } from './types.js';

export class HarnessesClient {
  constructor(private client: RokhaClient) {}

  async health(): Promise<{ status: string }> {
    return this.client.get('/api/harnesses/health');
  }

  async list(params?: { wallet_address?: string; harness_type?: string; tags?: string[] }): Promise<Harness[]> {
    const qs = new URLSearchParams();
    if (params?.wallet_address) qs.set('wallet_address', params.wallet_address);
    if (params?.harness_type) qs.set('harness_type', params.harness_type);
    if (params?.tags) qs.set('tags', params.tags.join(','));
    const query = qs.toString();
    return this.client.get(`/api/harnesses${query ? `?${query}` : ''}`);
  }

  async get(id: string): Promise<Harness> {
    return this.client.get(`/api/harnesses/${id}`);
  }

  async create(request: HarnessCreateRequest): Promise<Harness> {
    return this.client.post('/api/harnesses', request);
  }

  async update(id: string, request: HarnessUpdateRequest): Promise<Harness> {
    return this.client.put(`/api/harnesses/${id}`, request);
  }

  async delete(id: string): Promise<void> {
    await this.client.delete(`/api/harnesses/${id}`);
  }
}
