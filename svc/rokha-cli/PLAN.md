# Rokha CLI — PLAN

## Overview
All-in-one CLI + TUI for Rokha. Built in Rust. Binary name: `ro`.
Distributed via Homebrew tap, Linux installer, cargo, and npm.
First Rokha-branded tool on Rokha Registry.

## Status (v0.2.0)

### Shipped
- [x] Cargo project with clap arg parsing
- [x] Binary renamed `rokha` → `ro`
- [x] `ro version` — prints CLI + embedded schema version (3.0.0)
- [x] `ro status` — Erebus health + schema-drift check (alias: `ro health`)
- [x] `ro up` — docker pre-flight + embedded compose, brings up infra layer
      (postgres-erebus + postgres-agents + redis on `rokha-network`)
- [x] `ro down` — teardown
- [x] `ro logs [service]` — tail compose logs
- [x] `ro tools list` / `ro tools info <name>` — Rokha Registry
- [x] `ro chat <message>` — one-shot Rokha agent chat (routes to
      `/api/agents/rokha-agent/chat/public`)
- [x] `ro tui` — TUI dashboard skeleton
- [x] `ro mcp serve` — MCP server skeleton (`tools/list`)

### Constraints
`ro up` brings up the **infra layer only** today. Service binaries
(erebus, agents, harnesses, protocols) are not yet packaged as
images. For the full stack, run `just dev` from a Rokha checkout.
v0.3.0 will publish service images to GHCR and expand the embedded
compose.

## Phase 3 — Full stack `ro up` (v0.3.0)
- [ ] CI: build + publish service images to `ghcr.io/aetherbytes/rokha-*`
- [ ] Embed full compose (erebus, agents, harnesses, protocols)
- [ ] `ro init` to persist `.env.dev`-equivalent secrets

## Phase 4 — Auth + harnesses
- [ ] `ro login` — wallet challenge/verify or email
- [ ] `ro harnesses search/create/delete`
- [ ] Full chat TUI with conversation history
- [ ] `ro tools install <name>` — local fetch + cache

## Phase 5 — Advanced
- [ ] Workflow automation (`ro workflows run/export/import`)
- [ ] Plugin system for custom commands
- [ ] Performance profiling tools

## Installation (target — wired in this MVP's CI follow-up)

```bash
brew install aetherBytes/tap/rokha               # macOS
curl -fsSL https://get.rokha.ai/install.sh | sh  # Linux
cargo install rokha-cli                          # any platform with cargo
npm install -g @rokha/cli                        # any platform with npm
```

## Commands
```
ro version              Print CLI + schema version
ro status               Erebus health + schema drift check
ro up                   Launch local Rokha infra (docker required)
ro down                 Stop local Rokha infra
ro logs [service]       Tail compose logs
ro tools list           Browse Rokha Registry
ro tools info <name>    Tool details
ro chat <message>       One-shot chat with Rokha agent
ro tui                  TUI dashboard
ro mcp serve            MCP server (JSON-RPC over stdio)
```

## Rokha Registry Listing
- Type: Tool
- Category: infrastructure
- Provider: Rokha
- Tags: cli, mcp, local, tui, official
- Featured: yes (flagship)
