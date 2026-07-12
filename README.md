# Rokha SDK

Rokha is the **phone book — and the kitchen — of the agentic world**. Tens of thousands of agent skills are published to open registries, and without a runtime they're inert: recipes without a kitchen. Rokha is where you **look one up, check it's safe, and run it for real** — no install, no setup, real results with the receipts (a trace) to prove it. The runtime is the product.

Two kinds of users, one platform: **look-up users** find and try tools fast (free, no account); **builders** chain skills into workflows (also free), then sign in to keep, publish, and compose them. Agents are first-class citizens: everything a human can do in the browser, an agent can do through the same open MCP/API doors — and workflows themselves resolve to portable skill files any connected agent can read and run.

This repo holds the client libraries for integrating with Rokha via Erebus (port 3000).

**Building a tool of your own?** [docs/guides/scripted-skills.md](docs/guides/scripted-skills.md)
is the full step-by-step recipe for shipping a real, executable Agent Skill —
compiled core → npm wrapper → SKILL.md → Rokha registry → Rig — the same
pattern behind [Hoodwatch](https://github.com/aetherBytes/hoodwatch).

## Packages

| Package | Language | Path |
|---------|----------|------|
| `@rokha/sdk` | TypeScript | `typescript/` |
| `rokha-sdk` | Python | `python/` |

## API Surface

All requests route through **Erebus** (`localhost:3000`). No direct service connections.

| Path | Service | Description |
|------|---------|-------------|
| `/api/agents/*` | Agents (9003) | Chat, tasks, models |
| `/api/harnesses/*` | Harnesses (9004) | Memory/context CRUD |
| `/api/marketplace/*` | Rokha Registry | Listings, search, discovery |
| `/api/wallets/*` | Erebus | Auth, sessions |
| `/api/v1/chat/completions` | LLM Proxy | OpenAI-compatible |
| `/mcp/jsonrpc` | Protocols (8001) | MCP 2025-11-25 JSON-RPC |
| `/api/discovery/*` | Erebus | Service/tool discovery |
| `/api/skills/*` | Erebus | First-party [agentskills.io](https://agentskills.io) skills (public, CORS-open) |

## Quick Start (TypeScript)

```typescript
import { RokhaClient } from '@rokha/sdk';

const nb = new RokhaClient({ baseUrl: 'http://localhost:3000' });

const status = await nb.agents.status('rokha-agent');
const tools = await nb.mcp.listTools();
const harnesses = await nb.harnesses.list({ wallet_address: '...' });
```

## Quick Start (Python)

```python
from rokha import RokhaClient

nb = RokhaClient(base_url="http://localhost:3000")

status = nb.agent_status("rokha-agent")
tools = nb.mcp_list_tools()
harnesses = nb.list_harnesses(wallet_address="...")
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
curl https://erebus.rokha.ai/api/skills | jq

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
