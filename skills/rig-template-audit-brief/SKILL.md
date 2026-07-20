---
name: rig-template-audit-brief
description: A ready-made three-step Rokha Rig pattern — fetch → analyze → report. It takes ONE registry listing as its input: step 1 looks that listing up and fetches its real document, step 2 inspects the document for security and trust-boundary issues, step 3 writes a plain-language brief ending in an adopt / try-with-care / avoid verdict. Use when the user wants to vet a tool before adopting it, or wants a worked example of a fetch → analyze → report workflow. Instantiable by any agent over Rokha's public API, or one click in the Rokha editor.
license: MIT
compatibility: Works against any Rokha deployment (rokha.ai or self-hosted Erebus). Instantiation uses the public anon Working-Rig surface or the authenticated rigs API; running uses the run stream.
metadata:
  author: rokha
  version: "0.1.0"
  rokha_kind: rig-template
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Rig Template — Audit & Brief

A three-step workflow skeleton: **fetch → analyze → report**. The rig declares
one **input** — the registry listing you want vetted. Step 1 looks that listing
up and fetches its real document; step 2 inspects the document for security
issues — what it accesses, trust boundaries, risky patterns; step 3 writes the
brief a human actually wants to read, ending in a clear trust verdict.

Each step carries a registry *query*, resolved against the live Rokha
Registry at instantiation — the pattern adapts to the mesh's current
inventory.

## The pattern

Machine-readable skeleton: `assets/rig.json` (served at
`/api/skills/rig-template-audit-brief/assets/rig.json`, no auth):

- **Step 1 · fetch** — PINNED to Rokha Registry Search (`rokha-registry`),
  instructed to look up the listing named as the rig's input and fetch its
  real document (SKILL.md / server page) — the source material for the audit.
  Pinned rather than query-resolved because this step must be a known Rokha
  capability, not a fuzzy match.
- **Step 2 · analyze** — a security skill (registry query:
  `security audit code review`), instructed to inspect what it accesses,
  its trust boundaries, and risky patterns.
- **Step 3 · report** — a writing skill (registry query:
  `write report markdown`), instructed to write the audit brief with a
  clear trust verdict.

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

Identical to the flow documented in `rig-template-write-critique` — fetch
this template's `assets/rig.json`, resolve each step's `query` via
`GET /api/marketplace/registry?search=…`, create the rig + per-step
harnesses + memberships (anon mirror pre-login, `/api/rigs` authenticated),
then run via the run stream. Each step's output threads into the next; the
trace tree records the whole run.

## In the Rokha editor

RIG pane → **Start from a template** → Audit & Brief. Swap either step via
Add-to-Rig before running.
