---
name: rig-template-explore
description: A ready-made three-step Rokha Rig pattern — pick a listing from the Rokha Registry (a skill, harness, or rig), dissect its real-world use cases and best pairings, and write an explorer's brief. Use when the user wants to "check out a tool", understand what something in the registry is for, or wants a worked example of a lookup → analyze → brief workflow. Instantiable by any agent over Rokha's public API, or one click in the Rokha editor.
license: MIT
compatibility: Works against any Rokha deployment (rokha.ai or self-hosted Erebus). Instantiation uses the public anon Working-Rig surface or the authenticated rigs API; running uses the run stream.
metadata:
  author: rokha
  version: "0.1.0"
  rokha_kind: rig-template
  homepage: https://rokha.ai
  source_repo: https://github.com/aetherBytes/rokha-sdk
---

# Rig Template — Explore

A three-step workflow skeleton: **lookup → use-cases → brief**. Step 1 pulls
up a Rokha Registry listing — the one the user names, or a trending pick;
step 2 dissects what it's actually for (concrete workflows, what pairs well
with it from the registry, who it's for); step 3 writes the explorer's brief
a human actually wants to read.

Each step carries a registry *query*, resolved against the live Rokha
Registry at instantiation — the pattern adapts to the mesh's current
inventory.

## The pattern

Machine-readable skeleton: `assets/rig.json` (served at
`/api/skills/rig-template-explore/assets/rig.json`, no auth):

- **Step 1 · lookup** — a registry/discovery skill (query:
  `search registry discovery tools`), instructed to pull up everything about
  the listing: what it does, how it runs, who made it.
- **Step 2 · use-cases** — an analysis skill (query:
  `analyze documentation summarize`), instructed to name three concrete
  workflows it enables and the registry skills that pair well with it.
- **Step 3 · brief** — a writing skill (query: `write report markdown`),
  instructed to produce the plain-English explorer's brief.

## How an agent instantiates this (the public-API path)

Fetch this template's `assets/rig.json`, resolve each step's `query` via
`GET /api/marketplace/registry?search=…`, create the rig + per-step
harnesses (anon mirror pre-login, `/api/rigs` authenticated), then run via
the run stream. Each step's output threads into the next; the trace tree
records the whole run.

## In the Rokha editor

Builder → **template** → Explore. Swap any step via Add-to-Rig before
running.
