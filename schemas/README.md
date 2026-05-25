# Rokha Public API Schema

> The wire contract between Erebus (server) and the Rokha SDK (client).
> Single source of truth. Versioned. Drift is detectable at runtime.

## Files

| File | What |
|------|------|
| `openapi.yaml` | OpenAPI 3.1 spec covering every public Erebus endpoint. |
| `README.md` | This file — model, versioning rules, how to extend. |

## Why a schema layer

Erebus is Rust. The SDK is TypeScript + Python. Skills are Markdown.
Without a schema layer, all three drift independently — a field rename
in `rokha_audit` would surface as a runtime error in the SDK days after
release. With this schema layer:

- **One source of truth** that every implementation references.
- **Versioned** — semver-bumped on every contract change so clients can
  refuse to talk to incompatible servers.
- **Verifiable at runtime** — Erebus exposes `/api/schema` and
  `/api/schema/version`; the SDK can compare against its embedded copy
  on startup or before each request.
- **Generator-ready** — TS types via `openapi-typescript`, Python via
  `datamodel-code-generator`, Rust validation via `utoipa`. Not all
  wired up yet, but the spec is the input when we do.

## Versioning rule — non-negotiable

`openapi.yaml`'s `info.version` follows semver:

| Bump | When |
|------|------|
| **major** (1.0.0 → 2.0.0) | Removed endpoints, removed fields, type changes, response-shape changes that existing SDKs can't parse |
| **minor** (1.0.0 → 1.1.0) | New endpoints, new optional fields, expanded enum values |
| **patch** (1.0.0 → 1.0.1) | Documentation, examples, descriptions, schema-author fixes that don't change wire behavior |

Every PR that modifies `openapi.yaml`:

1. Bumps `info.version` per the table above.
2. Adds a line in [`CHANGELOG.md`](../CHANGELOG.md) (main repo) and in
   the SDK's `CHANGELOG.md` describing the change.
3. Mirrors the file to the public SDK repo (`aetherBytes/rokha-sdk/schemas/openapi.yaml`)
   in the same release. This is also encoded in CLAUDE.md's "Public
   Surface Contract" rule.

## Runtime sync verification

Erebus exposes:

```
GET /api/schema           → returns this openapi.yaml as text/yaml
GET /api/schema/version   → {"version": "1.0.0", "title": "...", "openapi": "3.1.0"}
```

The SDK can fetch `/api/schema/version` on first use and compare against
its embedded version string. Recommendation for SDK clients:

| Server major | Server minor.patch vs SDK | Behavior |
|--------------|----------------------------|----------|
| Same | Equal or server newer | Proceed silently |
| Same | SDK newer than server | Warn — server lacks features the SDK expects |
| Different | n/a | Refuse — incompatible wire format |

## Source-of-truth ownership

The **main repo's `schemas/openapi.yaml`** is canonical. The SDK repo
mirrors it. CI (when wired up) will fail a PR if `openapi.yaml`
changes in the main repo without a matching update in the SDK.

This is the same model as `skills/` — authored in main, mirrored to
SDK on each release. Both surfaces share the public-contract rule
defined in CLAUDE.md.

## Coverage today (v2.0.0)

| Resource | Paths | Status |
|----------|-------|--------|
| Meta | `/health`, `/api/info`, `/llms.txt`, `/api/schema`, `/api/schema/version` | ✅ |
| Skills | `/api/skills`, `/api/skills/{name}`, `/api/skills/{name}/SKILL.md`, `/api/skills/{name}/{path}` | ✅ |
| Wallets | `/api/wallets`, `/api/wallets/detect`, `/api/wallets/challenge`, `/api/wallets/verify`, `/api/wallets/status` | ✅ |
| Auth | `/api/auth/email/{register,login,verify,resend-verification}`, `/api/auth/google/{start,callback}`, `/api/auth/{token,refresh}` | ✅ |
| Agents | `/api/agents/health`, `/api/agents/rokha-agent/{chat,chat/public,chat/stream,chat/stream/public,status,tools,history,clear,model-info,available-models}` | ✅ |
| Harnesses | `/api/harnesses/health`, `/api/harnesses` (GET/POST), `/api/harnesses/search`, `/api/harnesses/{id}` (GET/PUT/DELETE), `/api/harnesses/{id}/{fork,publish}`, `/api/harnesses/wallet/{wallet}` (+`/{key}`) | ✅ |
| MCP | `/mcp/health`, `/mcp/tools`, `/mcp/jsonrpc` | ✅ |
| Marketplace | `/api/marketplace/{listings,listings/:id,search,featured,stats,registry,registry/:provider/:id,registry/stats,skills.md}` | ✅ |
| Discovery | `/api/discovery/{tools,agents,protocols,all,hot}` | ✅ |

## Coverage roadmap

| Resource | In schema? | SDK client? | Priority |
|----------|------------|-------------|----------|
| Meta + Skills + Wallets + Auth + Agents + Harnesses + MCP + Marketplace + Discovery | ✅ (v2.0.0) | ✅ existing | done |
| Sessions (`/api/agents/:name/sessions/*`) | ⏳ | partial | next minor |
| Streaming chat per-event schema (currently shape is doc-only) | ⏳ | partial | next minor |
| Content (`/api/content/*`) | ⏳ | ⏳ | when content surface stabilizes |
| LLM proxy (`/api/v1/chat/completions`, OpenAI-compatible) | ⏳ | ✅ existing | when shape audited |
| Rokha catalog (`/api/rokha/*`) | ⏳ | ⏳ | when Rokha catalog API stabilizes |

Adding a resource = add paths + components in `openapi.yaml`, bump
minor version, mirror to SDK, update CHANGELOG.

## Validation

Validate the spec is well-formed:

```bash
npx -y @redocly/cli@latest lint schemas/openapi.yaml
# or
npx -y @apidevtools/swagger-cli@latest validate schemas/openapi.yaml
```

Validate the live Erebus surface against the spec:

```bash
# Smoke each documented endpoint, confirm response matches schema
just verify-schema   # (planned — not yet wired into the justfile)
```

## Generating types (planned)

For TypeScript:
```bash
npx -y openapi-typescript@latest schemas/openapi.yaml \
  -o sdk/typescript/src/types.generated.ts
```

For Python:
```bash
pipx run datamodel-code-generator \
  --input schemas/openapi.yaml \
  --input-file-type openapi \
  --output sdk/python/src/rokha/types_generated.py
```

These are aspirational for now — the SDK has hand-authored types in
0.2.0. Once generated types prove out, we'll switch.
