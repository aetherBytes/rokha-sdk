import type { RokhaClient } from './client.js';

export interface CliDeviceStartRequest {
  scope?: string;
  client?: string;
}

export interface CliDeviceStartResponse {
  device_code: string;
  user_code: string;
  verification_uri: string;
  verification_uri_complete: string;
  expires_in: number;
  interval: number;
}

export interface CliDeviceIdentity {
  user_id: string;
  identity: string;
  auth_method: string;
  tier: 'free' | 'casual' | 'pro' | string;
}

export type CliDevicePollResponse =
  | { status: 'pending' }
  | { status: 'slow_down'; interval: number }
  | { status: 'authorized'; jwt: string; identity: CliDeviceIdentity }
  | { status: 'expired' }
  | { status: 'denied' };

export interface CliDeviceAuthorizeResponse {
  status: 'ok';
  device_summary: { client?: string | null; created_at: string };
}

export class CliAuthClient {
  constructor(private readonly client: RokhaClient) {}

  start(req: CliDeviceStartRequest = {}): Promise<CliDeviceStartResponse> {
    return this.client.post<CliDeviceStartResponse>('/api/auth/cli/start', req);
  }

  poll(device_code: string): Promise<CliDevicePollResponse> {
    return this.client.post<CliDevicePollResponse>('/api/auth/cli/poll', { device_code });
  }

  /** Requires `client.setWallet(...)` or a bearer-equipped fetch; sends JWT. */
  authorize(user_code: string): Promise<CliDeviceAuthorizeResponse> {
    return this.client.post<CliDeviceAuthorizeResponse>('/api/auth/cli/authorize', { user_code });
  }
}
