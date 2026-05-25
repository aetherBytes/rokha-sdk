import type { RokhaClient } from './client.js';

/**
 * Anthropic `/v1/messages` request shape — passed through verbatim to
 * the proxy. We surface only the common keys; everything else flows.
 */
export interface AnthropicMessagesRequest {
  model?: string;
  max_tokens?: number;
  system?: string | Array<Record<string, unknown>>;
  messages: Array<{
    role: 'user' | 'assistant';
    content: string | Array<Record<string, unknown>>;
  }>;
  temperature?: number;
  tools?: Array<Record<string, unknown>>;
  /** Must be false (or omitted) in v0.3.1 — streaming arrives in v0.3.2. */
  stream?: false;
  [extra: string]: unknown;
}

export interface AnthropicMessagesResponse {
  id: string;
  type: 'message' | string;
  role: 'assistant' | string;
  model: string;
  content: Array<Record<string, unknown>>;
  stop_reason: string | null;
  usage?: { input_tokens?: number; output_tokens?: number; [extra: string]: unknown };
  [extra: string]: unknown;
}

export class LlmClient {
  constructor(private readonly client: RokhaClient) {}

  /**
   * Forward an Anthropic-shaped message request through Rokha. The
   * caller's Bearer JWT is sent automatically (set via
   * `RokhaClient.setAuthToken`). The proxy uses the user's BYO
   * Anthropic key when present, or the Rokha tenant key (rate-limited).
   */
  proxy(req: AnthropicMessagesRequest): Promise<AnthropicMessagesResponse> {
    return this.client.post<AnthropicMessagesResponse>('/api/v1/llm/proxy', req);
  }
}
