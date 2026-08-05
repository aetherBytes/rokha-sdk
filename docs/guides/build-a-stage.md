# Build a Stage — dashboards and input for your rig

Every rig run on Rokha can ship a **dashboard**: visual output that renders on
the STAGE pane in the app and on the rig's public product page
(`rokha.ai/@handle/<rig-slug>`). This guide is written to be executable by an
agent over MCP (`https://rokha.ai/mcp/jsonrpc`) as well as by a human — every
path below names its exact call.

There are three tiers. **Always emit Tier 1**; add the others when the rig
deserves a designed or bespoke UI.

## Tier 1 — the `rokha_app` block (zero frontend code)

Make the rig's final step output one JSON object with a top-level `rokha_app`
key. Rokha renders it natively: title, verdict, 0–100 score, metric tiles,
markdown/table/graph sections, and up to 6 `actions` buttons. Full field spec:
the `RokhaApp` component in [the schema](https://rokha.ai/api/schema).

```json
{ "rokha_app": {
    "title": "X Signal — $DOGE",
    "verdict": "bullish",
    "score": 71,
    "metrics": [ { "label": "posts sampled", "value": "15", "tone": "ok" } ],
    "sections": [ { "heading": "Themes", "markdown": "- ecosystem growth — bullish" } ],
    "actions": [ { "id": "rerun", "label": "Scan another topic",
                   "input": { "label": "keywords", "hint": "$TICKER OR #tag" } } ]
} }
```

A JSON fence is fine; the parser also finds the block inside common envelopes.
Pressing an action fires a **fresh run billed to the presser** (runner-pays).

## Tier 2 — the designed stage template (`RokhaStageConfig`)

A **stored** HTML dashboard that ships on *every* run with zero per-run cost:
the runtime injects the chosen step's JSON into your template's
`__ROKHA_DATA__` slot and renders the result as the run's dashboard. The
config is the `RokhaStageConfig` component in the schema; it lives at
`content.stage` on the rig and travels through publish → adopt.

Set it over MCP (authenticated):

```json
{ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
  "params": { "name": "rig_stage_set", "arguments": {
    "rig_id": "<uuid>",
    "data_source": "step:Read sentiment",
    "title": "X Signal",
    "template_html": "<!doctype html><html><body><script>const DATA = __ROKHA_DATA__;</script>…</body></html>"
} } }
```

Rules (validated at save, same rules in the app's STAGE CONFIG panel):

- **Self-contained.** The stage frame blocks *all* network requests
  (CSP `default-src 'none'`). Inline every style and script; embed images as
  `data:` URIs. External `src`/`href` URLs are refused with the reason.
- **The slot is the mechanism.** `__ROKHA_DATA__` must appear at least once;
  every occurrence is replaced with the step's JSON, script-context-escaped
  (every `<` becomes `\u003c`) — hostile run data cannot break out of your
  script tag, and your own markup is untouched.
- **`data_source`**: `last_step` (default) or `step:<tag>` — use the named
  form when the final step is a bookkeeping step (e.g. a data-persist
  receipt) and the dashboard should show the analysis step's output.
- **Size**: template ≤400k chars; the injected document ≤600k. Over the cap
  the dashboard is dropped loudly (`stage_template_error` in the run result),
  never truncated — and the run itself always survives a broken template.
- **Precedence**: a run that emits its own `rokha_app`/`rokha-app.html`
  outranks the template. The template is the guarantee, not an override.

Read it back before editing with `rig_stage_get` (returns the template, its
`data_source`, and whether it currently validates).

## Tier 3 — the run-built artifact (`./rokha-app.html`)

A sandbox (scripted) step may write one self-contained file to
`./rokha-app.html` — the tool's own full UI, rebuilt from live data each run.
Same self-containment rules; ~500 KB cap, oversized is dropped. This is the
path for tools that ship their dashboard in their own binary (e.g. Solwatch's
`--out` HTML). Building from npm packages: bundle at *build time* and inline
the result — chart libraries fit (uPlot ~45 KB, Chart.js ~200 KB min) — or
have your sandbox step bundle with `npx -y esbuild --bundle` before writing
the file. Escape `</` inside embedded JSON.

## Input — how a stage takes it

- **Pre-run**: declare the rig's input contract (`rig_author`'s `input` /
  `inputs`) — that is the labelled box the STAGE and the product page render.
  A rig with an empty contract gives a visitor nothing to type into.
- **Post-run**: `rokha_app.actions[]` buttons, or — from inside a template or
  artifact — `window.parent.postMessage({ rokha: 'app_action', action: '<declared id>', input: '<value>' }, '*')`.
  Every press is a fresh adopt-and-run billed to the presser. There is no
  live socket into a run.

## Feeding downstream rigs

A dashboard shows the signal; a **data step** persists it. Give the rig a step
with `data_op: "append"` and `data_key: "data.<ns>.{{input}}"` — a later rig
reads the same key with `data_op: "read"` and the same input. Collections are
private, owner-scoped, 64 KB (append keeps the newest). The
`gust-collect-sentiment` → `gust-reply-decider` pair on
[@sage](https://rokha.ai/@sage) is the worked example — collector stages the
dashboard *and* persists the collection; the decider reads it later.
