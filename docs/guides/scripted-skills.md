# Build a scripted Agent Skill — compiled core → npm wrapper → Rokha Rig

This is the recipe behind [Hoodwatch](https://github.com/aetherBytes/hoodwatch)
(the Robinhood Chain memecoin auditor), written down so **any human or agent can
replicate it**. The shape:

```
your tool (Rust/Go/Zig… → ONE static binary)
   └─ public npm wrapper (ships the prebuilt binary; npx -y runs it anywhere)
        └─ SKILL.md (agentskills.io standard — portable to any agent)
             └─ Rokha registry listing (discoverable + runnable)
                  └─ Rig template (composed workflow other people run for real)
```

Why this shape? The Rokha sandbox (and most agent runtimes) can run
`npx -y <package> …` with **no toolchain, no API keys, no install step**. A
prebuilt static binary inside the npm package means the cold start is one
tarball download — not a compile. Your tool becomes a capability any agent can
execute for real, and every run leaves a trace.

Everything below is copy-pasteable. Where a step has a gotcha we hit for real,
it's called out.

---

## Step 1 — Write the tool as a single static binary

Any compiled language works. Rust is what we use (small binaries, easy static
linking, one `Cargo.toml`):

```toml
# Cargo.toml essentials
[package]
name = "yourtool"
version = "0.1.0"
edition = "2021"
license = "MIT"
repository = "https://github.com/you/yourtool"
readme = "README.md"
# ⚠ GOTCHA: pin the crate contents with a LEADING SLASH on each pattern.
# Unanchored patterns ("README.md") are gitignore-style and match at ANY
# depth — they'll vacuum node_modules/ docs and staged binaries into the
# crate and blow the crates.io 10 MB cap.
include = ["/src/**", "/Cargo.toml", "/README.md", "/LICENSE", "/tests/**"]

[profile.release]
opt-level = "z"   # size
lto = true
strip = true
codegen-units = 1
panic = "abort"
```

Design rules that make the tool agent-friendly:

1. **Every command supports `--json`.** Agents act on structured output;
   humans get the pretty renderer. One result type, serialized both ways.
2. **The JSON is self-describing.** Ship a stable result contract — e.g.
   `{ score, verdict, flags: [{id, severity, title, detail}], sections: {…} }`.
   Machine ids + severities on every finding, human strings alongside.
3. **Degrade, never fabricate.** A failed sub-check becomes a warning in the
   output — not invented data, and not a crash. Distinguish *"the upstream API
   failed"* from *"the answer is genuinely empty"*; they are different results
   and may deserve different severities.
4. **No secrets, no auth.** Read only public endpoints. If the tool needs
   credentials, take them as explicit user-supplied config — never bake them in.
5. **Guard untrusted input.** If an endpoint/URL is user-overridable, malformed
   responses must degrade, not panic (with `panic = "abort"`, one unchecked
   slice index kills the process).
6. **Budget for a sandbox wall clock.** Agent sandboxes run each command under
   a hard per-command timeout (Rokha's is ~30s). Two rules follow: **run
   independent network calls concurrently** (a chain of serial round-trips is
   the #1 way to blow the window — we cut one audit from ~40s to ~13s purely
   by joining fetches), and **ship a `--fast` mode** that skips the slowest
   passes so there's always a variant that fits.

Build it: `cargo build --release` → one binary, typically 2–4 MB.

## Step 2 — Wrap it in a public npm package

The wrapper is ~25 lines: a `bin.js` that picks the right prebuilt binary for
the platform and execs it.

```
npm/
├── package.json
├── bin.js
└── binaries/            ← filled by CI: yourtool-linux-x64, yourtool-linux-arm64,
                            yourtool-macos-arm64, yourtool-macos-x64, yourtool-windows-x64.exe
```

```json
{
  "name": "@you/yourtool",
  "version": "0.1.0",
  "license": "MIT",
  "bin": { "yourtool": "bin.js" },
  "files": ["bin.js", "binaries/", "SKILL.md", "README.md"],
  "engines": { "node": ">=18" }
}
```

```js
#!/usr/bin/env node
'use strict';
const { spawnSync } = require('node:child_process');
const { existsSync, chmodSync } = require('node:fs');
const path = require('node:path');

const PLAT = { linux: 'linux', darwin: 'macos', win32: 'windows' }[process.platform] || process.platform;
const ARCH = { x64: 'x64', arm64: 'arm64' }[process.arch] || process.arch;
const ext = process.platform === 'win32' ? '.exe' : '';
const bin = path.join(__dirname, 'binaries', `yourtool-${PLAT}-${ARCH}${ext}`);

if (!existsSync(bin)) {
  console.error(`yourtool: no prebuilt binary for ${PLAT}-${ARCH}.`);
  process.exit(1);
}
try { chmodSync(bin, 0o755); } catch {} // npm strips exec bits
const res = spawnSync(bin, process.argv.slice(2), { stdio: 'inherit' });
process.exit(res.status === null ? 1 : res.status);
```

The "fat package" approach (all platform binaries in one tarball) trades a few
MB of download for zero postinstall scripts and zero optionalDependencies
fragility — `npx -y` just works, including in offline npm caches.

## Step 3 — CI: tag → build matrix → publish

One workflow, fired by pushing a version tag. Order matters: **publish npm
first** (it's the load-bearing artifact — the thing agent sandboxes actually
run); the GitHub release and crates.io are `continue-on-error`.

```yaml
name: release
on:
  push: { tags: ["v*"] }
permissions: { contents: write }   # the GitHub-release step needs this

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          # musl static = runs on node:22-slim AND Alpine, no glibc dependency.
          - { os: ubuntu-22.04, target: x86_64-unknown-linux-musl,  out: yourtool-linux-x64 }
          # arm64 musl needs `cross` (a plain apt gcc-aarch64 can't link ring for musl)
          - { os: ubuntu-22.04, target: aarch64-unknown-linux-musl, out: yourtool-linux-arm64, cross: true }
          - { os: macos-14,     target: aarch64-apple-darwin,       out: yourtool-macos-arm64 }
          - { os: macos-14,     target: x86_64-apple-darwin,        out: yourtool-macos-x64 }
          - { os: windows-2022, target: x86_64-pc-windows-msvc,     out: yourtool-windows-x64.exe }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Install musl tools
        if: contains(matrix.target, 'musl') && !matrix.cross
        run: sudo apt-get update && sudo apt-get install -y musl-tools
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: "${{ matrix.target }}" }
      - uses: taiki-e/install-action@v2
        if: matrix.cross
        with: { tool: cross }
      - name: Build
        shell: bash
        run: |
          if [ "${{ matrix.cross }}" = "true" ]; then
            cross build --release --target ${{ matrix.target }}
          else
            cargo build --release --target ${{ matrix.target }}
          fi
      - name: Stage
        shell: bash
        run: |
          mkdir -p out
          src="target/${{ matrix.target }}/release/yourtool"
          [ -f "$src.exe" ] && src="$src.exe"
          cp "$src" "out/${{ matrix.out }}"
      - uses: actions/upload-artifact@v4
        with: { name: "${{ matrix.out }}", path: "out/${{ matrix.out }}" }

  publish:
    needs: build
    runs-on: ubuntu-22.04
    # ⚠ GOTCHA: the `secrets` context is not allowed in `if:` — map to env, gate on env.
    env:
      NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
      CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with: { path: staged }
      - name: Assemble npm binaries
        run: |
          mkdir -p npm/binaries
          find staged -type f -name 'yourtool-*' -exec cp {} npm/binaries/ \;
          cp SKILL.md README.md npm/ 2>/dev/null || true
          chmod +x npm/binaries/* || true
      - uses: actions/setup-node@v4
        with: { node-version: 20, registry-url: "https://registry.npmjs.org" }
      - name: Publish npm (FIRST — the load-bearing artifact)
        if: ${{ env.NPM_TOKEN != '' }}
        working-directory: npm
        env: { NODE_AUTH_TOKEN: "${{ secrets.NPM_TOKEN }}" }
        run: npm publish --access public
      - name: Attach binaries to the GitHub release
        if: startsWith(github.ref, 'refs/tags/')
        continue-on-error: true
        uses: softprops/action-gh-release@v2
        with: { files: "npm/binaries/*" }
      - uses: dtolnay/rust-toolchain@stable
      - name: Publish crate (optional)
        if: ${{ env.CARGO_REGISTRY_TOKEN != '' }}
        continue-on-error: true
        run: cargo publish --token "${{ secrets.CARGO_REGISTRY_TOKEN }}" --allow-dirty
        # --allow-dirty because npm/binaries/ was just staged into the tree —
        # this is exactly why Cargo.toml MUST pin `include` (Step 1), or the
        # crate tarball swallows the binaries and fails the 10 MB cap.
```

Add `NPM_TOKEN` (and optionally `CARGO_REGISTRY_TOKEN`) as repo Actions
secrets, then: `git tag v0.1.0 && git push origin v0.1.0`. Verify:

```bash
npm view @you/yourtool version
npx -y @you/yourtool --help
```

## Step 4 — Write the SKILL.md (the portable skill)

Follow the [agentskills.io](https://agentskills.io/specification) open standard:
a folder with a `SKILL.md` whose frontmatter has `name` + `description`, plus
progressive disclosure in the body. This is what makes the tool portable to
Claude Code, Claude.ai, the Agent SDK, Rokha, and any compliant agent.

```markdown
---
name: yourtool
description: One paragraph that tells an agent WHEN to use this — the concrete
  capability, the inputs it takes, and the trigger phrases ("use when someone
  wants to …"). This is the retrieval surface; write it for the router.
license: MIT
compatibility: A single self-contained binary with network egress — any
  sandbox, CI, or host. No API keys.
allowed-tools: Bash(npx:*)
metadata:
  author: you
  version: "0.1.0"
  npm: "@you/yourtool"
  source_repo: https://github.com/you/yourtool
  execution_class: scripted
---

# yourtool — what it does

## Quick start
    npx -y @you/yourtool run <input>

## Commands
(table of every command + flags)

## Output contract (for agents)
(the exact --json shape, as a commented example — this section is what lets
another agent chain your tool without reading your source)

## Known limitations (honest)
(what it does NOT check/do — agents plan around honest edges)
```

The **output contract** and **honest limitations** sections are the difference
between a tool agents can compose and a tool they have to guess at.

## Step 5 — Publish to the Rokha registry

Rokha's registry door is idempotent — publishing an existing slug under the
same owner **updates** it, so you can re-run on every release.

**Use the MCP door** (`POST /mcp/jsonrpc` with `registry_publish`). Humans and
agents both publish through it, and it derives the owner from your **verified
session token** — a caller cannot publish into someone else's account by putting
a different wallet in the body.

> ⚠ **GOTCHA (we hit this):** the REST endpoint
> `POST /api/marketplace/registry/publish` is **wallet-signature-gated** — a
> plain bearer token gets `403 signature_required`. Reach for the MCP door
> instead; it accepts the session token directly. Same result, one door.

```bash
# $JWT = your Rokha session token. Treat it like a password: never commit it,
# never paste it into a chat or an issue, and prefer a short-lived one.
curl -X POST https://rokha.ai/mcp/jsonrpc \
  -H "content-type: application/json" \
  -H "authorization: Bearer $JWT" \
  -d '{
    "jsonrpc": "2.0", "id": 1, "method": "tools/call",
    "params": {
      "name": "registry_publish",
      "arguments": {
        "name": "yourtool",
        "listing_type": "skill",
        "title": "YourTool",
        "description": "…the SKILL.md description…",
        "version": "0.1.0",
        "homepage": "https://github.com/example/yourtool",
        "tags": ["example", "scripted"],
        "metadata": {
          "slug": "yourtool",
          "skill_md": "…the full SKILL.md content…",
          "npm": "@example/yourtool",
          "execution_class": "scripted"
        }
      }
    }
  }'
```

No token yet? An agent can register itself and sign in **headlessly** with a
wallet keypair — `auth_wallet_challenge` → sign → `auth_wallet_verify` returns
the session token. `GET /llms.txt` documents the full flow.

Verify: `registry_search "yourtool"` (MCP) or
`GET /api/marketplace/registry?search=yourtool` — the second is public, no token.

## Step 6 — Compose it into a Rig (the workflow other people run)

A Rokha **Rig** chains your skill with instruction steps into a runnable
workflow. Ship a rig *template* next to the skill: a small `rig.json` skeleton
any agent can fetch and instantiate with its own input.

```jsonc
{
  "$schema": "https://rokha.ai/schemas/rig-template-v1.json",
  "name": "YourTool Audit",
  "intent": "One sentence: what the whole workflow produces.",
  "input": { "label": "what the user supplies", "kind": "text", "hint": "example value" },
  "steps": [
    {
      "role": "primary",
      "tag": "scan",
      "pin": { "name": "yourtool", "provider": "rokha", "slug": "yourtool" },
      "command": "npx -y @you/yourtool run {{input}} --json",
      "expects": "the input the user named",
      "instruction": "Run yourtool on the input. Return the full JSON.",
      "produces": "the raw result JSON"
    },
    {
      "role": "step",
      "tag": "brief",
      "expects": "the JSON from step 1",
      "instruction": "Turn the JSON into a plain-English brief. Use only what the tool reported — do not invent data.",
      "produces": "a human decision brief"
    }
  ]
}
```

The pattern to copy: **step 1 is your scripted skill** (the sandbox runs the
`npx` command for real), **step 2 is a pure instruction step** (the agent
reads step 1's trace output and writes for humans). Tool for truth, model for
prose.

How anyone instantiates it:
- **Human**: Rokha Builder → templates → your rig → type the input → Run.
- **Agent**: fetch the template's `rig.json`, recreate it via the `rig_author`
  MCP tool with their own input, execute through the run stream, read the
  traces.

## Step 7 — Give your tool a UI (without breaking the sandbox)

Your tool probably has a `serve` command, or you'd like the run to *show*
something rather than dump JSON. Here's the constraint that decides the design:

> **The sandbox your skill runs in is egress-only.** It can call out to the
> internet; **nothing can call in.** No browser — not yours, not Rokha's — can
> reach a web server running inside it. That isn't a limitation to route
> around; it's the wall that makes it safe to run untrusted code, and it does
> not move.

So a live `serve` inside a run is never viewable from outside. Instead, **the
run ships its frontend OUT as an artifact.** Two mechanisms, both of which any
tool can implement:

### a. The `rokha_app` block — a native view, no HTML at all

Have your tool's JSON output carry a top-level `rokha_app` key. Rokha's output
rail renders it natively — you write no frontend code, and there's no
tool-specific code in Rokha either:

```jsonc
{
  "rokha_app": {
    "title": "YourTool report — example-input",
    "verdict": "caution",                 // free-form status word
    "score": 54,                          // 0-100, optional
    "metrics": [                          // stat tiles; tone drives the color
      { "label": "Items scanned", "value": "1,204",  "tone": "neutral" },
      { "label": "Problems",      "value": "3",      "tone": "warn"    },
      { "label": "Critical",      "value": "0",      "tone": "good"    }
    ],
    "sections": [                         // markdown OR table per section
      { "heading": "Findings", "markdown": "- **Thing A** — explanation…" },
      { "heading": "Detail",   "table": {
          "columns": ["item", "status"],
          "rows": [["alpha", "ok"], ["beta", "failed"]] } }
    ]
  }
}
```

`tone` ∈ `good | neutral | warn | danger`. Point your Rig's final step at this
output and you get a dashboard. **This is the general contract — any rig that
emits `rokha_app` gets rendered, whatever it does.**

### b. A self-contained HTML artifact — your real UI, as a file

If you want *your* interface (charts and all), emit **one HTML file with the
data baked in**: CSS and JS inlined, no network requests, no external assets.
The simplest way is to reuse the exact page your `serve` command already
returns, injecting the run's JSON and stubbing out the fetch:

```js
// Same page `serve` renders; the data is embedded, so it needs no server.
const REPORT = { /* …the run's JSON… */ };
window.fetch = async () => ({ json: async () => REPORT });
```

The file renders anywhere — `file://`, a static host, or a hard-sandboxed
iframe — and regenerates on every run with the new input. The sandbox stays
sealed; you still get the pixels.

> ⚠ **GOTCHA — escape `</` when you embed JSON in a `<script>` tag.** A payload
> containing `</script>` (a token name, a URL, hostile input) closes the tag
> early and breaks the page — or worse, injects markup. Always
> `json.replace("</", "<\\/")` before embedding. Treat the JSON as untrusted
> even when it's your own tool's output: the data inside it came from the
> internet.

**Rule of thumb:** emit `rokha_app` always (it's cheap and it's what agents
read), and add the HTML artifact when your tool genuinely has a visual story to
tell.

---

## Security — the non-negotiables

Your skill runs on other people's accounts, on inputs you'll never see, and its
output is rendered in other people's browsers. Assume every input is hostile.

**Secrets**

- **Never bake a secret into the binary, the npm package, or `SKILL.md`.** They
  are all public artifacts. Anyone can `npm pack` your wrapper and read it.
- A skill that needs a key should take it from the **environment** and fail
  loudly with a typed error when it's missing — never fall back to a key you
  shipped. In Rokha, users attach their own keys by **alias**; the value is
  injected server-side at call time and never appears in your skill, the run
  receipt, or an agent's view.
- **The runner pays, and the runner supplies the credentials.** Never design a
  skill that spends *your* key on someone else's run.
- Publishing tokens (npm, crates.io, registries) live **only** in CI secrets.
  Scope them as narrowly as the registry allows, and never echo them in a build
  log. If one leaks, revoke it — don't rotate around it.

**Input and output**

- Every input from the network is hostile: an attacker controls token names,
  URLs, metadata, symbols. Validate before you interpolate — **never build a
  shell command by string-concatenating an input**. Pass arguments as an array;
  don't shell out through `sh -c`.
- Bound everything: request timeouts, retry counts, response sizes, loop
  iterations. A hostile endpoint that streams forever must hit a wall.
- **Escape before you embed** — the `</` rule above for HTML; parameterized
  queries only, never string-built SQL, for anything that touches a database.
- A malformed response must degrade into your output contract (a warning, a
  null field), **never a panic and never invented data**.

**Honesty**

- If your tool cannot do something for real, it must **say so and exit
  non-zero** — never emit plausible-looking fake output. A skill that
  role-plays execution is worse than one that fails: it launders a guess into
  something a person will act on. Rokha treats fabricated output as a bug in
  the skill.
- Document your blind spots in `SKILL.md`. "This check can't see X" is a
  feature; a silent gap is a liability.

**Least privilege**

- `allowed-tools` in `SKILL.md` should be the narrowest set that works (a
  scripted skill usually needs exactly `Bash(npx:*)`).
- Your binary should need **no** filesystem access outside its working
  directory and no inbound network. If it wants more, ask why.

---

## Step 8 — Run it for real and check the receipt

Every Rokha run writes a **trace**: the exact command, the tool's real output,
per step. That's the proof the skill executes rather than being role-played.
Test the full loop after each release:

```bash
# the exact command the sandbox runs — if this works, the rig works:
npx -y @you/yourtool run <real input> --json
```

then run the rig once in Rokha (Builder → Run, or `rig_author` + the run door
over MCP) and read the trace.

---

## Release-day gotchas (every one of these bit us)

- **A missing CI secret SKIPS the publish silently — it does not fail.** That's
  deliberate (binaries still build), but it means a green ✅ release can publish
  nothing at all. After your first tag, *check the registry*, not the checkmark:
  `curl -s https://registry.npmjs.org/@example/yourtool | head -c 100`.
- **A publish token scoped to one package can't create a NEW one.** If you reuse
  the token from an existing project and it was granular-scoped to *that
  package*, publishing a brand-new name 403s. Scope the token to the whole
  **org/scope**, not a single package.
- **CI's compiler is newer than yours.** Our release failed on a lint that
  didn't exist in the local toolchain (`-D warnings` turns any new lint into an
  error). Either pin the toolchain in CI, or expect to fix a lint you've never
  seen — don't assume a green local build means a green CI.
- **Keep every version string in lockstep** — the binary's manifest, the npm
  wrapper's `package.json`, and `SKILL.md`'s `metadata.version`. A wrapper that
  publishes `0.1.1` while the skill advertises `0.1.0` is a support ticket.
- **Nothing published? Then a bad tag costs nothing.** Fix forward and cut the
  next patch tag rather than force-moving a tag someone may already have.

---

## Checklist

- [ ] Binary builds static, ≤ a few MB, every command has `--json`
- [ ] Failures degrade into the output contract — no panics, no fabricated data
- [ ] No secret is baked into the binary, the npm package, or `SKILL.md`
- [ ] Inputs are validated; arguments passed as an array, never through `sh -c`
- [ ] Timeouts, retry caps, and size limits on every network call
- [ ] `Cargo.toml` pins `include = ["/src/**", …]` (leading slashes!)
- [ ] npm wrapper resolves every platform your CI actually builds
- [ ] CI: npm publish first; GitHub release + crates.io `continue-on-error`
- [ ] **Verified the package actually appeared on the registry** (green ✅ ≠ published)
- [ ] `npx -y @example/yourtool --help` works from a clean machine
- [ ] SKILL.md: router-grade description, output contract, honest limitations
- [ ] Published to the Rokha registry via the MCP door (idempotent — re-run per release)
- [ ] Rig template: scripted step + instruction step, `{{input}}` declared
- [ ] Emits `rokha_app` so the run renders as a view (+ an HTML artifact if it has a UI)
- [ ] `</` escaped anywhere JSON is embedded in HTML
- [ ] One real end-to-end run with a trace you've actually read

**Reference implementations — both are this recipe, live:**

- [aetherBytes/hoodwatch](https://github.com/aetherBytes/hoodwatch) — the
  Robinhood Chain memecoin auditor (Rust engine, npm wrapper, release workflow,
  SKILL.md, two rig templates).
- [aetherBytes/solwatch](https://github.com/aetherBytes/solwatch) — the Solana
  sister tool, which adds the **`rokha_app` block** and the **self-contained
  HTML artifact** from Step 7, plus the full data-vis dashboard.
