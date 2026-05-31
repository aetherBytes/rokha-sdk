# Changelog

All notable changes to the Rokha SDK — and notable updates to the
Rokha product it talks to — are documented here. The SDK is the public
face of Rokha; the wire contract it depends on is
`schemas/openapi.yaml`, served live at `/api/schema`.

## App — one cohesive look + expandable skill cards (2026-05-30)

_A frontend/app design pass on the pre-release Rokha web experience.
No SDK or wire-contract (`schemas/openapi.yaml`) change — nothing here
moves the SDK version._

The whole experience now feels like one product instead of a few
screens stitched together.

- **One calm, consistent look.** Chat, the live dashboard, and the
  Registry now share a single "steel & ice-blue" palette, with small,
  deliberate pops of color (cyan, green, violet) so it has life
  without feeling busy.
- **Skill cards work the same everywhere.** A tool or skill looks and
  behaves identically wherever you meet it — and opening one now
  **expands it in place**: in the Registry the card grows right in the
  grid and nudges its neighbors aside, instead of a small pop-up
  taking over the screen.
- **Easier to follow what Rokha is doing.** When Rokha works through a
  request, her steps read as a clean, scannable flow, and any tools or
  results she pulls back show up as tidy cards — not raw data dumps.
- **Bigger, clearer text** on large screens, and a tidier footer with
  evenly-sized controls.

**Where this gets us:** the experience of discovering tools, talking
to Rokha about them, and watching what she does now reads as one
coherent surface — another step toward the north-star journey
(discover → chat → build → keep it) feeling genuinely good to use.

**What's next:** turning the "build" step on — letting Rokha take a
tool you found and actually start composing something useful with it.

## App — Rokha chat, dashboard & registry polish (2026-05-30)

_A frontend/app update to the pre-release Rokha web experience. No SDK
or wire-contract (`schemas/openapi.yaml`) change — nothing here moves
the SDK version._

A round of UX work on the two halves of the experience: talking to
Rokha, and the live dashboard beside it.

- **Get back to your conversation in one tap.** A show/hide toggle for
  the chat lets you wander off into the registry or another panel and
  then flip straight back to your conversation with Rokha, instead of
  losing your place.
- **A calmer, easier-to-read chat.** The conversation now wears the
  same clean "glass" look as the rest of the dashboard, with clearer
  labels for who's speaking — Rokha or you.
- **Looks right on smaller screens.** On laptops and tablets the
  dashboard now drops the cramped desktop split for a simpler stacked
  layout — while keeping every panel and view you'd get on a big
  monitor. Nothing is removed on smaller screens; it just rearranges.
- **No more spill-over.** Fixed a visual glitch where cards inside the
  dashboard panels could bleed past the panel edge while scrolling.
- **Ask about anything in the registry, instantly.** Tapping a tool or
  skill in the Registry now drops a question about it straight into
  your chat with Rokha — no copy-paste, no losing your place.
- **Smoother, and easier on the eyes.** The animated background is
  lighter on your device, and the dashboard panels rest on a soft
  frosted layer so text stays crisp over the moving starfield behind it.

**Where this gets us:** chatting with Rokha and watching the live
dashboard now read as one coherent surface that holds up from a laptop
to a wide monitor — a real step toward the north-star journey (chat →
build → keep) feeling genuinely good to use, not just functional.

**What's next:** wiring up the remaining dashboard controls, making the
central orb react in real time to what Rokha is doing, and the
production hardening + public CLI/SDK release that gate a public launch.

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
