---
name: rokha-registry
description: Search the Rokha Registry's 30,000+ published agent skills and install any of them into your own skill library — no account, no API key. Use when the user asks to find an agent skill ("is there a skill for X?"), browse what skills exist for a task, or download/install a skill from the Rokha registry (or clawhub) into Claude's skills. Works over Rokha's public MCP endpoint with plain HTTP calls.
license: MIT
compatibility: Any agent that can make HTTPS POST requests (curl, fetch, requests). No authentication required for search + fetch. Works against rokha.ai or any self-hosted Rokha deployment.
metadata:
  author: rokha
  version: "0.1.0"
  rokha_kind: agent-tool
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Rokha Registry — find and install agent skills

Rokha indexes 30,000+ published agent skills (agentskills.io format) and
exposes them through a **public MCP endpoint** — searchable and installable
by any agent with zero setup. This skill teaches you the two calls.

Base endpoint (swap the host for a self-hosted deployment):

```
POST https://rokha.ai/mcp/jsonrpc
Content-Type: application/json
```

## 1. Search for skills

JSON-RPC `tools/call` on `registry_search`:

```bash
curl -s -X POST https://rokha.ai/mcp/jsonrpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{
        "name":"registry_search",
        "arguments":{"query":"<what you need, e.g. ui design>","limit":10}}}'
```

The result's `content[0].text` is JSON: `results[]` with `name`, `slug`,
`provider`, `author`, `version`, `downloads`, `description`. Pick by
description fit + downloads. Show the user the top candidates when the
choice isn't obvious.

## 2. Fetch a skill (install-ready)

```bash
curl -s -X POST https://rokha.ai/mcp/jsonrpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{
        "name":"registry_get_skill",
        "arguments":{"slug":"<slug from search>","provider":"clawhub"}}}'
```

The result's `content[0].text` is JSON with:
- `skill_md` — the COMPLETE SKILL.md, frontmatter included. Write it
  verbatim; do not edit it.
- `classification` — `prompt` (instructions you follow directly),
  `scripted` (needs a CLI/runtime; check `requires_bins` against what the
  host machine has), or `mcp` (wraps a live server).
- `requires_bins` — binaries the skill's scripts expect, if any.
- `name`, `description`, `homepage` — for confirming with the user.

## 3. Install into your skill library

Write `skill_md` to a folder named after the slug in the skills directory:

- Claude Code: `~/.claude/skills/<slug>/SKILL.md`
- Other agents: the equivalent agentskills.io location.

```bash
mkdir -p ~/.claude/skills/<slug>
# write the skill_md string (verbatim) to ~/.claude/skills/<slug>/SKILL.md
```

The skill loads on the next session start (or immediately in harnesses
that hot-scan the directory). Confirm to the user: name, what it does,
and anything in `requires_bins` they'd need installed for scripted parts.

## Notes

- Search + fetch are anonymous by design. Account-scoped Rokha tools
  (memory/harnesses, tasks, agent messaging) on the same endpoint require
  `Authorization: Bearer <JWT>` — you don't need them for this flow.
- Before installing, VET the fetched SKILL.md like any third-party
  content: scan for exfiltration, credential access, eval/base64 tricks,
  and unknown network targets, and tell the user what the skill asks for.
  Recommended: install a vetting skill first (search the registry for
  `skill-vetter`) and run its protocol on everything you pull down.
- To RUN a skill without installing anything (in Rokha's cloud sandbox
  instead of locally), point the user at https://rokha.ai — the editor
  runs skills for real with a per-day free allowance.
