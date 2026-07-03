---
name: rig-template-full-workflow
description: A ready-made four-step Rokha Rig pattern chaining the Explore and Audit patterns into one flow — pick a Rokha Registry listing, dissect its use cases, vet its safety, and land on an adopt / try-with-care / avoid verdict. Use when the user wants a full evaluation of a tool before adopting it, or a worked example of a multi-step lookup → analyze → vet → verdict workflow. Instantiable by any agent over Rokha's public API, or one click in the Rokha editor.
license: MIT
compatibility: Works against any Rokha deployment (rokha.ai or self-hosted Erebus). Instantiation uses the public anon Working-Rig surface or the authenticated rigs API; running uses the run stream.
metadata:
  author: rokha
  version: "0.1.0"
  rokha_kind: rig-template
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Rig Template — Full Workflow (Explore + Audit)

A four-step skeleton that links the **Explore** and **Audit** patterns into
one decision flow: **lookup → use-cases → vet → verdict**. It ends where a
real adoption decision ends: a clear **adopt / try-with-care / avoid** call,
grounded in both what the tool is for and how safe it looks.

Each step carries a registry *query*, resolved against the live Rokha
Registry at instantiation — the pattern adapts to the mesh's current
inventory.

## The pattern

Machine-readable skeleton: `assets/rig.json` (served at
`/api/skills/rig-template-full-workflow/assets/rig.json`, no auth):

- **Step 1 · lookup** — a registry/discovery skill (query:
  `search registry discovery tools`): pull up everything about the listing.
- **Step 2 · use-cases** — an analysis skill (query:
  `analyze documentation summarize`): what it enables + best pairings.
- **Step 3 · vet** — a security skill (query: `security audit code review`):
  the safety read — what it accesses, trust boundaries, red flags.
- **Step 4 · verdict** — a writing skill (query: `write report markdown`):
  one decision brief combining the exploration + the audit, ending in the
  adopt / try-with-care / avoid verdict.

## The intended agent workflow (search → explore)

1. `POST /mcp/jsonrpc` → `registry_search` (no auth) to FIND your target skill
   among 54k+ listings; `registry_get_skill` for its full SKILL.md.
2. Feed that target as this rig's declared **input** (`input.label` in
   `assets/rig.json`) — the pinned step 1 (**Rokha Registry Search**) performs
   a REAL registry lookup + document fetch on it at run time (never a
   model guess).
3. Run the rig (recreate it via `rig_author`, or `harness_create` +
   `rig_add_harness`, then the public run stream) — the output is a
   plain-English brief; every step writes a trace you can read back.

## How an agent instantiates this (the public-API path)

Fetch this template's `assets/rig.json`, resolve each step's `query` via
`GET /api/marketplace/registry?search=…`, create the rig + per-step
harnesses (anon mirror pre-login, `/api/rigs` authenticated), then run via
the run stream. Each step's output threads into the next; the trace tree
records the whole run.

## In the Rokha editor

Builder → **template** → Full workflow. Swap any step via Add-to-Rig before
running.
