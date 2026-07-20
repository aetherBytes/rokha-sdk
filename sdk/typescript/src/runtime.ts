import type { RokhaClient } from './client.js';

// The Runtime surface — "run for real". This is the product thesis: a skill is
// inert without a runtime, and these two doors are how a skill actually
// executes in the platform's isolated (egress-only, no-secrets) Fargate
// sandbox, writing a trace as the receipt.
//
// - `taste` is the PUBLIC free door: one real run per anon session per day,
//   inside the global daily budget. No JWT — the caller supplies an
//   `anon_session_id` (a UUID) which scopes the free run.
// - `run` is the LOGGED-IN door: draws from the user's tier daily quota. It
//   requires the client's bearer JWT (set via `RokhaClient.setAuthToken`).
//
// Both anchor their trace to a Working-Rig harness (`harness_id`, a UUID) —
// a run without an anchor can't persist, and the run IS its own receipt.

export interface TasteRequest {
  /** The caller's anon session id (same UUID the anon chat/rig uses). */
  anon_session_id: string;
  skill_provider: string;
  skill_slug: string;
  instruction?: string;
  params?: Record<string, unknown>;
  /** The Working Rig's harness id (UUID) — anchors the trace. */
  harness_id: string;
}

export interface RunRequest {
  skill_provider: string;
  skill_slug: string;
  instruction?: string;
  params?: Record<string, unknown>;
  /** The Working Rig's harness id (UUID) — anchors the trace. */
  harness_id: string;
  /** Optional model pin for the run. */
  model?: string;
}

/** A run acknowledgement. Field set varies; `run_id` identifies the run so you
 *  can poll progress and read the resulting trace. */
export interface RunAck {
  run_id?: string;
  status?: string;
  [extra: string]: unknown;
}

export class RuntimeClient {
  constructor(private readonly client: RokhaClient) {}

  /**
   * The free public taste — one real sandbox run per anon session per day.
   * No auth; pass the anon session id in the body.
   */
  taste(req: TasteRequest): Promise<RunAck> {
    return this.client.post<RunAck>('/api/runtime/taste', req);
  }

  /**
   * A logged-in real run against the caller's tier daily quota. Requires the
   * bearer JWT (set via `RokhaClient.setAuthToken`).
   */
  run(req: RunRequest): Promise<RunAck> {
    return this.client.post<RunAck>('/api/runtime/run', req);
  }

  /**
   * Poll a run's live progress (public, run-token gated server-side).
   * Returns whatever progress the server exposes for `run_id`.
   */
  progress(runId: string): Promise<unknown> {
    return this.client.get(`/api/runtime/runs/${runId}/progress`);
  }
}
