import type { RokhaConfig, HealthResponse } from './types.js';
import { AgentsClient } from './agents.js';
import { CliAuthClient } from './cli-auth.js';
import { HarnessesClient } from './harnesses.js';
import { LlmClient } from './llm.js';
import { MCPClient } from './mcp.js';
import { MarketplaceClient } from './marketplace.js';
import { RigsClient } from './rigs.js';
import { WalletsClient } from './wallets.js';

const DEFAULT_BASE_URL = 'http://localhost:3000';
const DEFAULT_TIMEOUT = 30_000;

export type SchemaCompatLevel = 'match' | 'minor-drift' | 'major-drift' | 'unreachable';

export interface SchemaCompatReport {
  level: SchemaCompatLevel;
  client: string;
  server?: string;
  message: string;
}

export interface SkillMdBundle {
  ok: boolean;
  provider: string;
  slug: string;
  name: string;
  description?: string;
  homepage?: string;
  /** prompt = a model can execute it faithfully; scripted/mcp = needs a runtime. */
  classification: 'prompt' | 'scripted' | 'mcp';
  classification_basis?: string;
  requires_bins?: string[];
  has_install_steps?: boolean;
  scripts_referenced?: string[];
  frontmatter_metadata?: Record<string, unknown> | null;
  body: string;
  body_chars?: number;
  truncated?: boolean;
  cached?: boolean;
}

export class RokhaClient {
  static readonly SCHEMA_VERSION = '4.6.0';

  readonly baseUrl: string;
  readonly timeout: number;
  private walletAddress?: string;
  private walletChain?: string;
  private authToken?: string;

  readonly agents: AgentsClient;
  readonly cliAuth: CliAuthClient;
  readonly harnesses: HarnessesClient;
  readonly llm: LlmClient;
  readonly mcp: MCPClient;
  readonly marketplace: MarketplaceClient;
  readonly rigs: RigsClient;
  readonly wallets: WalletsClient;

  constructor(config: Partial<RokhaConfig> = {}) {
    this.baseUrl = (config.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, '');
    this.timeout = config.timeout ?? DEFAULT_TIMEOUT;
    this.walletAddress = config.walletAddress;
    this.walletChain = config.walletChain;

    this.agents = new AgentsClient(this);
    this.cliAuth = new CliAuthClient(this);
    this.harnesses = new HarnessesClient(this);
    this.llm = new LlmClient(this);
    this.mcp = new MCPClient(this);
    this.marketplace = new MarketplaceClient(this);
    this.rigs = new RigsClient(this);
    this.wallets = new WalletsClient(this);
  }

  /** Set the bearer JWT used by `RokhaClient.fetch()` for protected routes. */
  setAuthToken(jwt: string | undefined): void {
    this.authToken = jwt;
  }

  setWallet(address: string, chain = 'solana'): void {
    this.walletAddress = address;
    this.walletChain = chain;
  }

  async fetch(path: string, init: RequestInit = {}): Promise<Response> {
    const url = `${this.baseUrl}${path}`;
    const headers = new Headers(init.headers);

    if (!headers.has('Content-Type') && init.body) {
      headers.set('Content-Type', 'application/json');
    }
    if (this.walletAddress) {
      headers.set('x-wallet-address', this.walletAddress);
    }
    if (this.walletChain) {
      headers.set('x-wallet-chain', this.walletChain);
    }
    if (this.authToken && !headers.has('Authorization')) {
      headers.set('Authorization', `Bearer ${this.authToken}`);
    }

    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), this.timeout);

    try {
      const res = await globalThis.fetch(url, {
        ...init,
        headers,
        signal: init.signal ?? controller.signal,
      });
      if (!res.ok) {
        const body = await res.text().catch(() => '');
        throw new RokhaError(res.status, body, path);
      }
      return res;
    } finally {
      clearTimeout(timer);
    }
  }

  async get<T = unknown>(path: string): Promise<T> {
    const res = await this.fetch(path);
    return res.json() as Promise<T>;
  }

  async post<T = unknown>(path: string, body?: unknown): Promise<T> {
    const res = await this.fetch(path, {
      method: 'POST',
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    return res.json() as Promise<T>;
  }

  async put<T = unknown>(path: string, body?: unknown): Promise<T> {
    const res = await this.fetch(path, {
      method: 'PUT',
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });
    return res.json() as Promise<T>;
  }

  async delete(path: string): Promise<void> {
    await this.fetch(path, { method: 'DELETE' });
  }

  async health(): Promise<HealthResponse> {
    return this.get('/health');
  }

  /**
   * Ingest a registry listing's real SKILL.md (schema 4.6.0): fetched from
   * its source registry, parsed, and classified server-side. The
   * `classification` answers "can a model execute this faithfully?" —
   * `prompt` yes; `scripted`/`mcp` need a runtime. No auth.
   */
  async getSkillMd(provider: 'clawhub' | 'rokha', slug: string): Promise<SkillMdBundle> {
    const q = new URLSearchParams({ provider, slug });
    return this.get(`/api/marketplace/registry/skill-md?${q}`);
  }

  /**
   * Verify the live Erebus schema version matches what this SDK was built against.
   * Returns a report; the SDK does not throw on drift — caller decides.
   */
  async checkSchemaCompat(): Promise<SchemaCompatReport> {
    const client = RokhaClient.SCHEMA_VERSION;
    let server: string;
    try {
      const res = await this.get<{ version: string }>('/api/schema/version');
      server = res.version;
    } catch (_e) {
      return {
        level: 'unreachable',
        client,
        message: `Could not fetch ${this.baseUrl}/api/schema/version`,
      };
    }
    if (server === client) {
      return { level: 'match', client, server, message: 'OK' };
    }
    const [smaj] = server.split('.');
    const [cmaj] = client.split('.');
    if (smaj !== cmaj) {
      return {
        level: 'major-drift',
        client,
        server,
        message: `Incompatible: server ${server}, SDK ${client}. Upgrade @rokha/sdk.`,
      };
    }
    return {
      level: 'minor-drift',
      client,
      server,
      message: `Server ${server}, SDK ${client}. Forward-compatible; some endpoints may be new.`,
    };
  }
}

export class RokhaError extends Error {
  constructor(
    public readonly status: number,
    public readonly body: string,
    public readonly path: string,
  ) {
    super(`Rokha API error ${status} on ${path}: ${body}`);
    this.name = 'RokhaError';
  }
}
