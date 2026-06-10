import type { RokhaClient } from './client.js';

// Rigs + Traces — the Working Rig surface (schema 4.1.0).
// A Rig is a workflow of configured harnesses; a trace is the record of a
// run (input in, result out). The authed surface is owner-scoped via the
// client's bearer JWT. Pre-login, the SAME surface is mirrored under
// /api/anon/... keyed by an `x-anon-session-id` header — get that view via
// `client.rigs.anon(sessionId)`.

export interface TraceRecord {
  id: string;
  harness_id?: string | null;
  rig_id?: string | null;
  trace_kind?: string;
  status: string;
  input?: unknown;
  result?: unknown;
  metadata?: Record<string, unknown> | null;
  created_at?: string;
}

export class RigsClient {
  constructor(
    private client: RokhaClient,
    private prefix = '/api',
    private extraHeaders: Record<string, string> = {},
  ) {}

  /** The pre-login mirror of this surface, scoped to an anon session id. */
  anon(sessionId: string): RigsClient {
    return new RigsClient(this.client, '/api/anon', { 'x-anon-session-id': sessionId });
  }

  private async req<T>(method: string, path: string, body?: unknown): Promise<T> {
    const res = await this.client.fetch(`${this.prefix}${path}`, {
      method,
      headers: this.extraHeaders,
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    return res.json() as Promise<T>;
  }

  // --- Rigs ---

  async list(): Promise<unknown> {
    return this.req('GET', '/rigs');
  }

  async create(rig: { key?: string; summary?: string; content?: Record<string, unknown> }): Promise<unknown> {
    return this.req('POST', '/rigs', rig);
  }

  async get(id: string): Promise<unknown> {
    return this.req('GET', `/rigs/${id}`);
  }

  async update(id: string, body: Record<string, unknown>): Promise<unknown> {
    return this.req('PUT', `/rigs/${id}`, body);
  }

  async listHarnesses(rigId: string): Promise<unknown> {
    return this.req('GET', `/rigs/${rigId}/harnesses`);
  }

  async addHarness(
    rigId: string,
    member: { harness_id: string; position?: number; role?: string },
  ): Promise<unknown> {
    return this.req('POST', `/rigs/${rigId}/harnesses`, member);
  }

  // --- Traces ---

  async listTraces(limit = 50, offset = 0): Promise<{ data?: TraceRecord[] } & Record<string, unknown>> {
    return this.req('GET', `/traces?limit=${limit}&offset=${offset}`);
  }

  async getTrace(id: string): Promise<unknown> {
    return this.req('GET', `/traces/${id}`);
  }

  async createTrace(trace: {
    harness_id?: string;
    rig_id?: string;
    trace_kind?: 'atomic' | 'run';
    input?: unknown;
    result?: unknown;
    status?: 'success' | 'error' | 'partial' | 'running';
    metadata?: Record<string, unknown>;
  }): Promise<unknown> {
    return this.req('POST', '/traces', trace);
  }
}
