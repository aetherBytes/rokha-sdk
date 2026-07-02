---
name: rig-template-audit-brief
description: A ready-made two-step Rokha Rig pattern — step 1 audits/inspects a tool or claim, step 2 turns the findings into a plain-language brief with a verdict. Use when the user wants to "check something out and summarize it", vet a tool and get a readable report, or wants a worked example of an analyze → report workflow. Instantiable by any agent over Rokha's public API, or one click in the Rokha editor.
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

A three-step workflow skeleton: **fetch → analyze → report**. Step 1 fetches
the target (a Rokha Registry listing, a URL, a repo, or an endpoint); step 2
inspects it for security issues — what it accesses, trust boundaries, risky
patterns; step 3 writes the brief a human actually wants to read, ending in
a clear trust verdict.

Each step carries a registry *query*, resolved against the live Rokha
Registry at instantiation — the pattern adapts to the mesh's current
inventory.

## The pattern

Machine-readable skeleton: `assets/rig.json` (served at
`/api/skills/rig-template-audit-brief/assets/rig.json`, no auth):

- **Step 1 · fetch** — a fetch skill (registry query:
  `fetch url http request`), instructed to gather the target's source
  material — registry listings welcome.
- **Step 2 · analyze** — a security skill (registry query:
  `security audit code review`), instructed to inspect what it accesses,
  its trust boundaries, and risky patterns.
- **Step 3 · report** — a writing skill (registry query:
  `write report markdown`), instructed to write the audit brief with a
  clear trust verdict.

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
