---
name: rokha-audit
description: Security and compliance audit for MCP tools, agents, and skills found in the wild. Run before installing or invoking any unfamiliar tool. Produces a clear "safe to use / here's how" or "found these vulnerabilities" verdict with the schema, endpoint, install command, and trust signals the agent or user needs to make a decision. Use when the user mentions auditing, vetting, security-checking, or "is this safe" about an MCP tool, ClawHub skill, or Smithery server.
license: MIT
compatibility: Requires Rokha's `rokha_audit` MCP tool (served by rokha-protocols on port 8001 in dev, or Rokha's public MCP endpoint in prod). Stage 2 probe is optional and benefits from a sandboxed Node environment (Rokha's cloud runtime sandbox, Claude Code's bash, or any local Node).
metadata:
  author: rokha
  version: "0.1.0"
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Null Audit

A three-stage audit that tells the user whether an MCP tool, skill, or agent
is safe to invoke and how to call it correctly.

## When to use this skill

Activate when the user wants to evaluate a third-party tool before using it.
Trigger phrases include:

- "audit X" / "is X safe" / "vet X"
- "what does X do" (when X is unfamiliar)
- "should I install X"
- A new tool name appearing in a workflow without trust context

If the user just wants to *use* a tool they already trust, this skill is the
wrong call — invoke the tool directly.

## Inputs

| Input | Required | Source |
|-------|----------|--------|
| Tool name | Yes | User message, slash command (`/audit <name>`), or `/.nb/audit-target.json` in a Rokha session |
| Probe consent | Optional | Stage 2 only runs in a sandboxed env *and* with implicit or explicit user consent. Skip silently if no sandbox. |

## Stage 1 — Heuristic scan (always runs)

Call Rokha's `rokha_audit` MCP tool with `{tool_name}`. The server-side
implementation looks the tool up in the marketplace registry, then runs six
checks on the metadata:

- **SCHEMA** — does it declare an `inputSchema` with required parameters?
- **ENDPOINT** — is there a declared endpoint, and is it HTTPS?
- **PERMISSIONS** — does the description mention sensitive operations (shell, exec, sudo, write, delete, credential)?
- **METADATA** — author present, verified by registry, download count > 0?
- **INSTALL** — install command shape (PASS for npm/npx, FAIL for curl-pipe-bash)
- **TRUST** — homepage/repository present, stars > 0?

The MCP tool returns markdown with a check table, an overall pass/warn/fail
count, and a risk level (LOW/MEDIUM/HIGH).

**If risk is HIGH or any check is FAIL, stop here and report. Do not run
Stage 2** — the goal is to keep the user safe, not to discover more about a
tool that already failed basic checks.

## Stage 2 — Live probe (only in a sandbox)

Only run this stage if:
1. The user is in a sandboxed environment (Rokha's cloud runtime sandbox, a fresh container, or explicit `--probe` flag), AND
2. Stage 1's risk is LOW or MEDIUM with no FAILs.

What to do:

- **HTTPS endpoint** (registry declares `endpoint: https://…`): POST `tools/list` via Rokha's `/api/marketplace/mcp-proxy` endpoint. No local install needed.
- **npm-installable** (registry declares `metadata.install` or `metadata.npm_package`, OR user passes `--npm <pkg>` to override): inside the sandbox, `npm install --no-save <pkg>` then spawn it via `npx -y <pkg>`, send JSON-RPC `initialize` + `tools/list` over stdio, capture the response, kill the process. See `references/probe-stdio-mcp.md` for the exact recipe.
- **Otherwise**: skip Stage 2 with note "probe skipped — no endpoint or npm install method." Registry items frequently lack explicit install metadata; the `--npm` override lets the user opt in when they know the package name.

Capture: list of exposed tools, their input schemas, install duration,
postinstall hook content (if any), and any errors observed (install
failure, protocol mismatch, timeout). Results are persisted as
`probe_results` in the harness — see `references/audit-harness-schema.md`.

### CLI invocation patterns

```bash
# Auto-detect from registry metadata
nb audit Math-MCP

# Force npm probe with explicit package
nb audit Math-MCP --npm @smithery-ai/math-mcp

# Show what got saved
nb audit --config
```

## Stage 3 — Persist findings

Save a structured harness so the user (or another agent in their workspace)
can recall it later:

- Free / authenticated tier → single slot at key `tool.audit.latest`
- Casual / pro tier → per-tool at key `tool.audit.{normalized_name}`
- Tags: `["tool", "audit", "nb-audit"]`
- Body: see `references/audit-harness-schema.md`

Increment `run_count` on re-audit; don't duplicate.

## Output to present to the user

Render in this order. Be specific — the user is making a decision, not
reading a report.

```
[verdict emoji] <Tool name> — <verdict>

Risk: <level>          (e.g. MEDIUM — usable with caveats)
Source: <registry>     (e.g. smithery, clawhub, github)

What it does
  <one sentence from the registry description>

Inputs
  <param>: <type>   <description if known>
  ...
  (or "no input schema declared" if missing)

How to use safely (only if verdict is OK or OK-with-caveats)
  <example invocation — endpoint URL, npx command, or MCP server config>

Concerns (only if any check WARNed or FAILed)
  - <check name>: <reason, plus a remediation if obvious>

Saved to harness: <key>
```

## Edge cases

- **Tool not in registry**: return "no such tool found in Rokha,
  ClawHub, or Smithery. Use exact name from the registry homepage." Do
  not guess.
- **Multiple matches**: prefer exact case-insensitive name match. If none,
  prefer the highest-download tool from the search results, and surface the
  ambiguity in the output.
- **Probe fails with network error**: don't claim the tool is broken —
  most likely the sandbox can't reach the host. Note it and keep going.
- **Audit re-run with same tool name within 1 hour**: read the existing
  harness, mention "previous result: X, re-running" so the user knows we're
  not silently caching.

## Related

- Heuristic scan source of truth: Rokha MCP server (`rokha_audit` tool)
- Stage 2 sandbox: Rokha's cloud runtime sandbox (Fargate, on rokha.ai) or
  any local Node + sandbox env
- Findings persistence: Rokha harnesses (wallet-scoped, private)
- Cross-agent portability: this skill ships as a standalone SKILL.md
  folder; drop it in `~/.claude/skills/`, `~/.openclaw/workspace/skills/`,
  or any agentskills.io-compatible client
