---
name: rig-template-write-critique
description: A ready-made two-step Rokha Rig pattern — step 1 writes a piece (any writing skill from the live registry), step 2 critiques it against the actual output. Use when the user wants to "write and review", "draft then critique", or wants a worked example of chaining two skills into one workflow. Instantiable by any agent over Rokha's public API, or one click in the Rokha editor.
license: MIT
compatibility: Works against any Rokha deployment (rokha.ai or self-hosted Erebus). Instantiation uses the public anon Working-Rig surface or the authenticated rigs API; running uses the run stream.
metadata:
  author: rokha
  version: "0.1.0"
  rokha_kind: rig-template
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Rig Template — Write & Critique

A two-step workflow skeleton: **create → review**. Step 1 produces a piece of
writing; step 2 receives step 1's *actual output* (Rokha threads each step's
result into the next) and critiques it with a grade.

This is a **template, not a pinned toolchain**: each step carries a registry
*query*, and instantiation resolves it against the live Rokha Registry — so
the pattern re-shapes around whatever the mesh offers today.

## The pattern

Read the machine-readable skeleton from `assets/rig.json` (served at
`/api/skills/rig-template-write-critique/assets/rig.json`, no auth):

- **Step 1 · create** — a writing skill (registry query: `writing`), with a
  default instruction to produce the requested piece.
- **Step 2 · review** — a critique/review skill (registry query:
  `review critique feedback`), instructed to critique the previous step's
  output and grade it.

## How an agent instantiates this (the public-API path)

1. `GET /api/skills/rig-template-write-critique/assets/rig.json` — the skeleton.
2. For each step, pick a real skill:
   `GET /api/marketplace/registry?search=<step.query>&limit=5` and choose the
   best match (downloads are a fair tiebreak).
3. Create the rig + harnesses. Pre-login, use the anon mirror with an
   `x-anon-session-id` header (one step only — multi-step needs an account);
   authenticated, use `/api/rigs` + `/api/harnesses`:
   - rig: `{ key: "working-rig", summary: <template name>, content: { intent: <rig.json intent> } }`
   - one harness per step: `{ harness_type: "skill", key: "working-rig.target" | "working-rig.h2", content: { skill: {name, source}, endpoint: "", tool: "", params: {}, instruction: <step.instruction> } }`
   - membership per step: `POST /api/rigs/:id/harnesses { harness_id, position, role: <step.role> }`
   - mirror the steps into the rig's `content` (`target`, `target_harness_id`, `config`, `extra_harnesses`) so every surface hydrates.
4. Run it: `POST /api/agents/rokha-agent/run/stream/public` (anon) or
   `/api/agents/rokha-agent/run/stream` (bearer JWT). Each step's output
   threads into the next; the run records a parent `run` trace with per-step
   atomic traces (`GET /api/traces`).

## In the Rokha editor

One click: RIG pane → **Start from a template** → Write & Critique. The
editor resolves the queries against the registry and fills the workbench;
swap any step via Add-to-Rig before running.
