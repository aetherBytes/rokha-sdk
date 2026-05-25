# Changelog

All notable changes to the Rokha SDK are documented here. The SDK is
the public face of Rokha; the wire contract it depends on is
`schemas/openapi.yaml`, served live by Erebus at `/api/schema`.

## 0.5.0 — Federated LLM proxy (slice 1 of LLM routing)

**Additive. Schema bumped 3.1.0 → 3.2.0 (minor — 1 new endpoint). SDK 0.4.0 → 0.5.0.**

Adds typed bindings for the new federated LLM proxy. The proxy lets
any authenticated Rokha caller (CLI, SDK, third-party agent) forward
Anthropic-shaped `/v1/messages` requests through Rokha — using their
own BYO Anthropic key (if stored in their account) or the Rokha
tenant key with the free-tier daily rate limit.

### Added

- **`RokhaClient.llm`** (TS) / **`llm_proxy()` method** (Py):
  - `llm.proxy(body)` → forwards to `POST /api/v1/llm/proxy`. Body
    is the Anthropic messages request; response is Anthropic's
    response shape verbatim.
- TS exports `LlmClient` + `AnthropicMessagesRequest` +
  `AnthropicMessagesResponse` from index.
- `SCHEMA_VERSION` bumped to `3.2.0` in both SDKs.

### Wire contract (schema 3.2.0)

- New tag `llm`.
- New path `POST /api/v1/llm/proxy` with `bearerAuth` security.
- New components: `AnthropicMessagesRequest` + `AnthropicMessagesResponse`
  (intentionally permissive — `additionalProperties: true` — so the
  proxy stays a pass-through as Anthropic's API evolves).

### Server policy (Erebus)

- Caller's user-scoped BYO Anthropic key (via `agent_api_keys`) wins
  when present and bypasses the rate limit.
- Otherwise the tenant fallback is `ANTHROPIC_KEY_ROKHA_AGENT` env
  (then `ANTHROPIC_API_KEY`), and the existing free-tier daily limit
  (`LLM_DAILY_RATE_LIMIT`, default 100) applies — same shape as the
  agent chat routes.
- Only `claude-*` models. Default `claude-haiku-4-5-20251001`.
- `stream=true` is rejected (slice 2 / v0.3.2 adds streaming).

### What this unlocks

- Third-party SDK callers can chat with Anthropic via Rokha (the
  user's account fronts the compute). They never see the key.
- Slice 2 — `rokha-agents` repoint to call the proxy instead of
  Anthropic directly — lands next session. After that, `ro up`
  works end-to-end without `ANTHROPIC_KEY_ROKHA_AGENT` on the
  local machine.

## 0.4.0 — CLI device flow (`ro login`)

**Additive. Schema bumped 3.0.0 → 3.1.0 (minor — 3 new endpoints). SDK 0.3.0 → 0.4.0.**

Adds typed bindings for the CLI device authorization grant (RFC 8628
adapted) that powers `ro login`. Full design:
[docs-internal/src/operations/cli-device-flow.md](../docs-internal/src/operations/cli-device-flow.md).

### Added

- **`RokhaClient.cliAuth`** (TS) / **`cli_auth_*` methods** (Py) — typed
  wrappers around the new endpoints:
  - `cliAuth.start({ scope?, client? })` → `CliDeviceStartResponse`
  - `cliAuth.poll(device_code)` → discriminated `CliDevicePollResponse`
    (`pending` / `slow_down` / `authorized` + jwt + identity / `expired` / `denied`)
  - `cliAuth.authorize(user_code)` — requires bearer JWT; binds the
    code to the caller's identity, mints a 30-day CLI-scoped JWT.
- **`RokhaClient.setAuthToken(jwt)`** (TS) / **`set_auth_token(jwt)`**
  (Py) — sets the Bearer token used on protected routes.
- TS exports `CliAuthClient` + the four response interfaces from index.

### Wire contract

- Schema 3.1.0 adds `/api/auth/cli/{start,poll,authorize}` and four
  components: `CliDeviceStartResponse`, `CliDevicePollResponse`
  (oneOf with `discriminator: status`), `CliDeviceAuthorizeResponse`.
- `SCHEMA_VERSION` bumped to `3.1.0` in both SDKs.

### Notes

- The CLI `ro login` ships in `rokha-cli` 0.3.0 (separate package).
- The Hecate `/cli` browser page is not yet shipped — until it is,
  the user must complete the `authorize` step by `curl`ing the
  endpoint with their existing session JWT. See the design doc.

## 0.3.0 — Schema drift detection wired

**Additive. Schema stays at 3.0.0. SDK 0.2.0 → 0.3.0.**

Closes the gap noted in the 0.2.0 release notes: SDK clients can now
verify they're talking to a compatible Erebus before sending requests.

### Added

- **`RokhaClient.SCHEMA_VERSION`** (TS class static / Py class attr) —
  the schema version this SDK build was compiled against (`3.0.0`).
- **`RokhaClient.checkSchemaCompat()`** / **`check_schema_compat()`** —
  fetches `/api/schema/version` and returns a `SchemaCompatReport`:
  - `match` — server and SDK agree
  - `minor-drift` — same major, server has a newer minor (forward-compatible)
  - `major-drift` — incompatible (SDK should refuse)
  - `unreachable` — schema endpoint did not respond
- TS: `SchemaCompatLevel` and `SchemaCompatReport` exported from index.
- Py: `SchemaCompatReport` dataclass exported from `rokha`.

The SDK does not throw on drift — callers decide whether to warn,
refuse, or proceed.

### Companion: rokha-cli 0.2.0

Same schema-version constant lives in the `ro` CLI (binary renamed
from `rokha` → `ro`). `ro status` runs the same drift check against
the configured Erebus.

## 0.2.0 — Agent rebrand: Hecate/Hex → Rokha Agent

**BREAKING. Schema bumped 2.0.0 → 3.0.0 (major). SDK 0.1.0 → 0.2.0.**

The resident agent is no longer "Hecate"/"Hex" — it is **Rokha**, the
breath that animates the flow. The agent and the framework now share
the name by design.

### Breaking changes

- **Agent routes renamed.** Every `/api/agents/hecate/*` endpoint is
  now `/api/agents/rokha-agent/*` (chat, chat/public, chat/stream,
  chat/stream/public, status, tools, history, clear, model-info,
  available-models). The old `/api/agents/hecate/*` paths are **gone**
  — there are no compatibility aliases. Callers must update.
- **Agent identifier.** The agent's id/dispatch token is now
  `rokha-agent` (was `hecate`). Any code passing `'hecate'` as the
  agent argument must pass `'rokha-agent'`.
- **MCP tool names.** Agent-owned MCP tools exposed at `/mcp/jsonrpc`
  are renamed `hecate_*` → `rokha_agent_*` (e.g. `hecate_remember` →
  `rokha_agent_remember`, `hecate_new_session` →
  `rokha_agent_new_session`). Any MCP client invoking these by name
  must update.
- **Discovery insight field.** `seed_hex` → `seed_rokha_agent` in the
  discovery insight payload.
- **TypeScript:** `AgentsClient.hecateChat()` → `rokhaAgentChat()`;
  all `agent` defaults are now `'rokha-agent'`.
- **Python:** `RokhaClient.agent_status()` / `agent_tools()` (and
  peers) now default `agent="rokha-agent"`.

### Migration

- Replace `/api/agents/hecate/` → `/api/agents/rokha-agent/` and the
  agent token `'hecate'` → `'rokha-agent'`.
- Replace MCP tool names `hecate_*` → `rokha_agent_*`.
- Server-side deployments: run the included DB migrations
  (erebus `011`, harnesses `008`, agents `003`) so seeded keys, rate
  limits, SYSTEM personality, and per-user model choice carry over.

### Notes

- `RokhaClient.SCHEMA_VERSION` / `nb.checkSchemaCompat()` are not yet
  implemented in this SDK (pre-existing gap, unrelated to this
  release); the canonical schema version is `info.version: 3.0.0` in
  `schemas/openapi.yaml`, served at `/api/schema/version`.
