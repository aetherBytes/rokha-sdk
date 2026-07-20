# Rokha SDK

Rokha is the **phone book — and the kitchen — of the agentic world**. Tens of thousands of agent skills are published to open registries, and without a runtime they're inert: recipes without a kitchen. Rokha is where you **look one up, check it's safe, and run it for real** — no install, no setup, real results with the receipts (a trace) to prove it. The runtime is the product.

Two kinds of users, one platform: **look-up users** find and try tools fast (free, no account); **builders** chain skills into workflows (also free), then sign in to keep, publish, and compose them. Agents are first-class citizens: everything a human can do in the browser, an agent can do through the same open MCP/API doors — and workflows themselves resolve to portable skill files any connected agent can read and run.

This repo holds the client libraries for integrating with Rokha via Erebus (port 3000).

**Found a bug, or have an idea?** See
[docs/guides/feedback.md](docs/guides/feedback.md) — TL;DR:
[open an issue](https://github.com/aetherBytes/rokha-sdk/issues/new) if you're a
developer, or use the **🐞 Report a bug** link in the rokha.ai footer if you'd
rather not have a GitHub account (we file it for you).

**Building a tool of your own?** [docs/guides/scripted-skills.md](docs/guides/scripted-skills.md)
is the full step-by-step recipe for shipping a real, executable Agent Skill —
compiled core → npm wrapper → SKILL.md → Rokha registry → Rig — the same
pattern behind [Hoodwatch](https://github.com/aetherBytes/hoodwatch).

## Packages

| Package | Language | Path | What it is |
|---------|----------|------|------------|
| `@rokha/sdk` | TypeScript | `sdk/typescript/` | Typed HTTP client |
| `rokha-sdk` | Python | `sdk/python/` | Typed HTTP client |
| `ro` | Rust (CLI) | `svc/rokha-cli/` | The local terminal agent |

> **Distribution status:** the `ro` CLI ships as a prebuilt binary via
> [`scripts/install.sh`](scripts/install.sh) and the GitHub Releases. The
> language packages (`@rokha/sdk` on npm, `rokha-sdk` on PyPI, `rokha-cli` on
> crates.io) and the Homebrew tap are **not published yet** — until then, use
> the CLI installer or build from source in this repo.

## The local terminal agent (`ro`)

`ro` is a lightweight **local interface** to Rokha: a thin client on your
machine that does no heavy lifting itself — discovery, execution, and the agent
all run on the platform. Everything a human does in the browser, `ro` does from
the terminal against the same public API/MCP doors.

```bash
ro status                 # remote reachability + schema match + login state
ro login                  # browser device-flow sign-in (RFC 8628)
ro tools list <query>     # search the live Rokha Registry (~54k listings)
ro chat "<message>"       # one-shot chat with the Rokha agent
ro tui                    # live heads-up monitor (status / registry / agent)
ro mcp serve              # expose the FULL platform tool suite to any MCP host
```

**`ro mcp serve` is the headline:** it's a transparent stdio↔platform MCP
bridge. Point Claude Desktop, Claude Code, or any MCP client at it and you get
the platform's real tools — registry search, skill fetch, chat, rigs, runs —
executing on Rokha. Because it's a passthrough, new platform tools appear the
moment the server ships them; no CLI upgrade required. Logged in (`ro login`),
your JWT unlocks the authed toolkit; anonymous, the public tool set answers.

## API Surface

All requests route through **Erebus** — one front door. Point the SDK at
`https://api.rokha.ai` for the live platform, or `http://localhost:3000` for a
local dev stack. No direct service connections.

| Path | Service | Description |
|------|---------|-------------|
| `/api/agents/*` | Agents | Chat, tasks, models |
| `/api/harnesses/*` | Harnesses | Memory/context CRUD |
| `/api/marketplace/registry` | Rokha Registry | Listings, search, discovery |
| `/api/runtime/*` | Runtime | Run a skill for real (free taste + tier-quota run) |
| `/api/wallets/*` | Erebus | Auth, sessions |
| `/api/v1/chat/completions` · `/api/v1/models` | LLM Proxy | OpenAI-compatible |
| `/api/v1/llm/proxy` | LLM Proxy | Anthropic `/v1/messages`-shaped |
| `/mcp/jsonrpc` | MCP | MCP 2025-11-25 JSON-RPC (public + authed tools) |
| `/api/discovery/*` | Erebus | Service/tool discovery |
| `/api/skills/*` | Erebus | First-party [agentskills.io](https://agentskills.io) skills (public, CORS-open) |

## Quick Start (TypeScript)

```typescript
import { RokhaClient } from '@rokha/sdk';

const nb = new RokhaClient({ baseUrl: 'https://api.rokha.ai' });

const status = await nb.agents.status('rokha-agent');
const tools = await nb.mcp.listTools();
const results = await nb.marketplace.search({ query: 'UI design' });

// Run a skill for real (free public taste — one per anon session/day):
const ack = await nb.runtime.taste({
  anon_session_id: crypto.randomUUID(),
  skill_provider: 'rokha',
  skill_slug: 'some-skill',
  harness_id: crypto.randomUUID(),
});
```

## Quick Start (Python)

```python
from rokha import RokhaClient

nb = RokhaClient(base_url="https://api.rokha.ai")

status = nb.agent_status("rokha-agent")
tools = nb.mcp_list_tools()
results = nb.search_listings(query="UI design")
```

## Skills (cross-agent portability)

Rokha publishes [agentskills.io](https://agentskills.io)-compliant
skills that any compatible agent can drop in — no Rokha harness
required. The skills live alongside this SDK at
[`skills/`](https://github.com/aetherBytes/rokha-sdk/tree/main/skills)
and are served live at `GET /api/skills` on every Erebus instance.

### Browse available skills

```bash
# Live registry (CORS-open, no auth)
curl https://api.rokha.ai/api/skills | jq

# Or the public GitHub mirror
curl https://api.github.com/repos/aetherBytes/rokha-sdk/contents/skills | jq '.[].name'
```

### Install a skill

For Claude Code (`~/.claude/skills/`):
```bash
mkdir -p ~/.claude/skills/rokha-audit
curl -fsSL https://raw.githubusercontent.com/aetherBytes/rokha-sdk/main/skills/rokha-audit/SKILL.md \
  -o ~/.claude/skills/rokha-audit/SKILL.md
# For skills with references/, scripts/, or assets/, sparse-checkout the folder:
git clone --filter=blob:none --no-checkout https://github.com/aetherBytes/rokha-sdk.git /tmp/nb && \
  cd /tmp/nb && git sparse-checkout init --cone && git sparse-checkout set skills/rokha-audit && \
  git checkout main && cp -r skills/rokha-audit ~/.claude/skills/
```

For OpenClaw (`~/.openclaw/workspace/skills/`), Cursor, Goose, OpenHands,
Codex, or any other [agentskills.io-compatible client](https://agentskills.io)
— same drop-in pattern, just point at that client's skills directory.

### Available skills

| Skill | What it does |
|-------|--------------|
| [`rokha-audit`](https://github.com/aetherBytes/rokha-sdk/tree/main/skills/rokha-audit) | Security & compliance audit for MCP tools. Three-stage flow: heuristic scan → optional sandboxed probe → harness persistence. |

### The layering, briefly

| Layer | What | How to consume |
|-------|------|----------------|
| **Distribution** | SKILL.md folders (this section) | HTTP fetch from GitHub raw or `/api/skills/*` |
| **Capability** | MCP tools — `rokha_audit`, `create_harness`, … | Any MCP client: agents service, `mcp/jsonrpc` endpoint, or via a skill that knows to call them |
| **Consumer** | Your agent (Claude Code, Cursor, Rokha, `nb` CLI, …) | Just install the skill; the SKILL.md tells your agent what to do |

You don't need OpenClaw. You don't need Rokha's frontend. You don't
need npm. Skill ≠ package. Drop a folder, your agent runs the workflow.
