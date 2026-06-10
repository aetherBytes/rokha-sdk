# Changelog

All notable changes to the Rokha SDK — and notable updates to the
Rokha product it talks to — are documented here. The SDK is the public
face of Rokha; the wire contract it depends on is
`schemas/openapi.yaml`, served live at `/api/schema`.

## Product — chain two skills, no login (2026-06-10)

_No SDK or schema changes._

Visitors can now chain a **second step** onto their workflow before
signing in — step 1's output feeds step 2, so you feel composition
actually working, not just read about it. Ready-made templates seed both
steps too. The third step (and naming, saving, automation) is where the
account comes in.

## Product — Remix takes requests (2026-06-10)

_No SDK or schema changes._

Remix — the "watch Rokha build a workflow, live" button — now takes
requests. Type **`/remix`** followed by anything ("summarize",
"humanizer", "weather") and Rokha finds the best matching skill on the
mesh and builds with *that*, narrating every step. Plain `/remix` keeps
the surprise-me random pick. Works before you log in. (Builders get the
same power programmatically: pass a target with the remix call.)

## Product — Start Building, one click from any skill (2026-06-10)

_No SDK or schema changes._

The **Start Building** button on every skill card is live: click it and
the skill becomes your workflow's harness with the Editor open and ready
— discovery to building in one motion, no login needed.

## 0.7.6 — Memory records gain an always-load switch (2026-06-10)

_Schema 4.7.0 (additive) · SDK 0.7.6 (TypeScript + Python)._

Harness records (Rokha's memory layer) now carry an explicit
`always_load` flag: `true` means the record is injected into the agent's
context every turn (core identity, your profile); `false` (the default)
means it's retrieved on demand by relevance search. Previously this
split was hardwired into the agent — now it's data on the record, so
what the agent always knows vs. looks up is declared, inspectable, and
editable. Create/update/search all accept it; no SDK method changes —
versions bumped in lockstep.

## Product — every skill now tells you if it really runs (2026-06-10)

_No SDK or schema changes — the 4.6.0 ingestion endpoint, now visible._

Open any skill in the Registry and you'll see its **execution badge**,
derived from the skill's actual definition file (not its marketing
blurb): ⚡ **Runs here now** — instruction-shaped skills the model can
execute faithfully, today — or 🔒 **Needs the Rokha runtime** — skills
that depend on command-line tools or installs, listed by name, honest
about being a demonstration until the hosted runtime arrives. The detail
panel also now shows the skill's real definition file, so what you read
is what runs.

## 0.7.5 — Skills become real, readable tools (2026-06-10)

_Schema 4.6.0 (additive) · SDK 0.7.5 (TypeScript + Python)._

A skill listing used to be a card — a name and a short blurb. Now Rokha
reads the actual skill:

- **New endpoint: `GET /api/marketplace/registry/skill-md`** — fetches a
  listing's real SKILL.md from its source registry, parses it, and
  returns a structured tool definition: the full instructions, what
  binaries it needs, what scripts it references, and an execution
  **classification** — `prompt` (a model can run it faithfully, today),
  `scripted` / `mcp` (needs a runtime — the honest "demonstration" label
  until the Rokha runtime arrives).
- **Runs use the real thing.** When you run a skill in the Editor, the
  engine now executes the skill's *actual* full instructions — not the
  card blurb — and the needs-a-runtime call is made from the skill's own
  declared requirements, not a guess.
- **SDK:** `getSkillMd(provider, slug)` (TS) / `get_skill_md(provider,
  slug)` (Python).

## Product — the Editor opens, and runs get honest (2026-06-10)

_No SDK or schema changes — a product update worth knowing about._

- **The Editor is now open to everyone — no login needed.** Search the
  registry, pick a skill, configure it, run it, and read the full record
  of what happened, straight from the browser. Sign-in is only needed
  for the deeper stuff (saving named workflows, multi-step libraries).
- **Runs now tell you the truth.** Skills backed by a live endpoint run
  for real, as always. Skills that are really instructions for a model
  (writing, critique, analysis) run for real too — the model *is* the
  engine, and the result says so. And skills that need real tooling
  (command-line programs, installs, external fetching) no longer
  pretend: you get a clearly-labeled **demonstration** of what the
  output looks like, never invented results.
- **Where this is going:** a Rokha-hosted runtime that actually executes
  those tool-backed skills for you — no installs, no setup. The
  demonstration label is the placeholder for that button.

## 0.7.4 — Privacy-safe product analytics (2026-06-10)

_Schema 4.5.0 (additive) · SDK 0.7.4 (TypeScript + Python)._

Rokha now measures itself the way it treats your data — privately:

- **New capture surface** (`/api/analytics/*`): anonymous, session-keyed
  visit and interaction capture. Attribution is random UUIDs only — never
  wallet addresses, tokens, or raw IPs — and the client honors Do Not
  Track. This powers "is the product working" questions (visits, feature
  adoption) without third-party trackers; nothing leaves the Rokha stack.
- No SDK method changes — versions bumped in lockstep.

## 0.7.3 — Swap a workflow step by asking (2026-06-10)

_Schema 4.4.0 (additive) · SDK 0.7.3 (TypeScript + Python)._

Started from a template and step 2 picked a tool you don't love? Now you
just say so:

- **New MCP tool: `rig_swap_skill`.** Re-resolves one step of a rig
  against the live registry and re-points it — the step's instruction is
  kept, and any live endpoint/params from the old tool are safely
  cleared. Ask Rokha "use a different tool for step 2" and the editor
  updates in place; external agents get the same single call over MCP.
- Rokha's per-turn briefing now lists every step of a multi-step rig, so
  she addresses steps by number from real state.
- No SDK method changes — versions bumped in lockstep.

## 0.7.2 — Agents can fetch templates themselves (2026-06-10)

_Schema 4.3.0 (additive) · SDK 0.7.2 (TypeScript + Python)._

The rig templates that landed below are now reachable from *inside* the
agent mesh, not just over REST:

- **New MCP tools: `skills_list` + `skills_read`.** Any agent connected
  over MCP (`/mcp/jsonrpc`) — including Rokha herself in chat — can list
  the first-party skills catalog (filter by kind, e.g. `rig-template`),
  read a skill's SKILL.md, or pull a template's `assets/rig.json`
  skeleton, then build the rig with the existing harness/rig tools. Ask
  Rokha "start me from a template" and she does the whole loop herself.
- No SDK method changes — the bump keeps `SCHEMA_VERSION` in lockstep
  with the served contract.

## Skills — Rig templates: start a workflow from a pattern (2026-06-10)

_New first-party skills; no wire-contract change._

Two **rig templates** join the first-party skills catalog — ready-made
workflow patterns any agent (or the editor's one-click "Start from a
template") can instantiate against the live Registry:

- **Write & Critique** — step 1 writes, step 2 critiques the actual output.
- **Audit & Brief** — step 1 inspects, step 2 writes the plain-language
  verdict.

Each template is a standard skill whose `assets/rig.json` carries the
skeleton (per-step registry queries + default instructions), and whose
SKILL.md documents the full public-API instantiation flow — so the same
pattern works for a human in the editor and an external agent over the API.

## 0.7.1 — Chain steps into real workflows (2026-06-10)

_Schema 4.2.0 (additive) · SDK 0.7.1 (TypeScript + Python)._

The build loop grows up: with an account you can now **chain multiple
tools into one workflow** — and watch the whole thing run as a single,
threaded flow.

- **Multi-step Rigs.** Your first tool leads the flow; "+ add harness"
  chains the next step. Each step has its own instruction and settings,
  and you can reorder or remove steps freely.
- **Threaded runs.** Run the Rig and each step's output feeds the next —
  ask step one to write something and step two to critique it, and the
  critique is about the *actual* output. The record is a run trace with
  each step's atomic trace nested under it.
- **My Rigs.** Name your Rig, keep several, and load any of them back
  into the workbench. Whichever you touched last is the one Rokha sees
  as "your rig."
- **Wire contract:** the authenticated run stream is now documented
  (schema 4.2.0, additive). Guests keep the full single-step loop.

## 0.7.0 — Build it, run it, read the trace (2026-06-10)

_Schema 4.1.0 (additive) · SDK 0.7.0 (TypeScript + Python)._

The big one: the **full build loop now works end to end, before you even
log in**. Pick any tool from the Registry, drop it into your Rig, give it
an instruction, hit **Save & Run** — Rokha executes it and the result
lands as a permanent, readable record (a *trace*).

**In the app:**

- **The Editor is now part of the main screen.** No separate sandbox —
  the Editor tab lays your workbench out right next to the chat: your Rig,
  a live map of its structure, Rokha's thinking, and a searchable Registry
  pane so you never have to leave to go find a tool.
- **Configure your harness.** Targeting a skill now opens a config panel:
  tell Rokha what to do with it (the instruction), optionally point it at
  a live tool server (it probes what's really there and builds the input
  form from the server's actual contract), and set parameters.
- **Run anything.** Tools with a live server get called for real. Tools
  without one — most of the catalog — Rokha runs *herself*, performing the
  skill and producing its output. Either way you get a trace.
- **Traces — your run history.** A new view lists every run with its full
  input and output. One tap asks Rokha to dissect what came back.
- **Remix goes all the way now.** The "watch Rokha build" button no longer
  stops after picking a tool — she discovers, targets, configures, runs,
  and reads the result, live, end to end.
- **Ask Rokha to build it for you.** Every config field has an "ask Rokha
  to fill" button — she edits the same Rig you see, and the screen updates
  as she works.

**In the SDK / wire contract (additive — your 4.0.0 code keeps working):**

- New documented endpoints: recent mesh activity, the guarded MCP
  probe/call proxy, the full **Rigs + Traces** surface (including its
  pre-login mirror), and the two live agent streams (Remix and Run).
- TypeScript: new `client.rigs` (with `.anon(sessionId)` for the pre-login
  mirror), `marketplace.discoverRecent()`, `marketplace.mcpProxy()`.
- Python: `list_rigs` / `get_rig` / `create_rig` / `update_rig` /
  `list_traces` / `get_trace` (all with an optional `anon_session_id`),
  `discover_recent()`, `mcp_proxy()`.

## App — A refreshed landing + a guided "Agent Skills" walkthrough (2026-06-09)

_A frontend/app pass. No SDK or wire-contract (`schemas/openapi.yaml`)
change — nothing here moves the SDK version._

The homepage got a ground-up refresh, and Rokha can now walk you through
the basics herself:

- **A cleaner first screen.** When you arrive in the void, a single
  floating **"What is Rokha?"** card explains the idea, with quick buttons
  to see what's trending, take a tour, or watch Rokha build something live
  (Remix). One tap on "What is Rokha?" gets you a plain-English, no-jargon
  explainer.
- **Built for phones.** The whole landing was reworked for mobile — the
  quick-start buttons lay out cleanly around the screen, tuck extras into
  tidy dropdowns to save space, and the footer keeps Docs / X / Contact as
  compact icons. Nothing important gets lost on a small screen.
- **A tidier top bar on phones.** The menu now holds the main destinations
  (Rokha, the Registry, the Editor), and the feed button moves up next to
  it — so the wasted gap beside the logo is gone and the quick-start
  buttons sit neatly under the bar. One less row of clutter, more room for
  the actual screen.
- **New guided tutorial — Agent Skills.** A short, narrated walkthrough of
  the thing everything here is built from: what a *skill* is, the open
  `SKILL.md` standard that lets **any** agent use it, and how you compose
  your own workflow from skills with **no code and no API keys**. It even
  pulls up a real trending skill so you can see one live.
- **Polish throughout** — a deeper, more tactile look for the floating
  void elements, highlights that stay locked onto whatever Rokha is
  pointing at (even while you scroll on a phone), and a calmer default
  view.
- **On phones, Rokha stays out of your way.** When you ask her something
  on a small screen, she no longer flips you over to a data view and away
  from what you were reading — she answers in chat and just points you to
  the feed if there's more to see. (On desktop, where the views sit beside
  the chat, she still pulls them up for you.)

## Docs — FAQ + Common Terms reference pages (2026-06-09)

_A docs pass. No SDK or wire-contract (`schemas/openapi.yaml`) change._

Two new plain-language reference pages, linked from the homepage:

- **FAQ** — what Rokha is, why come here, whether you need an account
  (you don't, to look around), and how to start.
- **Common Terms** — a glossary of the core building blocks (Skill,
  Harness, Rig, Trace) plus the words you'll see around the platform
  (the mesh, the Registry, Remix, the dashboard, and more).

## App — Rokha points right at what she means (2026-06-06)

_A frontend/app pass. No SDK or wire-contract (`schemas/openapi.yaml`)
change — nothing here moves the SDK version._

When Rokha talks about a skill, she now highlights it right where you're
reading — in her live thinking view — instead of flashing an unrelated
list. A few touches make her guidance easier to follow:

- **One clear highlight that waits for you.** Her "look here" cue uses a
  single, consistent accent color and **stays put until you engage** —
  hover it, click it, or just move on — rather than blinking away before
  you've found it.
- **Chat about any skill in one click.** Skill cards now carry a quick
  **"ask Rokha"** button, so you can dig into a skill without opening it
  up first.
- **Remix spotlights its pick.** When Rokha builds a Rig for you, she
  highlights the exact tool she chose (and only the latest one), so it's
  obvious what landed in your workflow.
- **A tidier dashboard default** plus small polish across the panels.

## App — a big step toward building workflows with agents (2026-06-04)

_A large product release. No breaking SDK or wire-contract
(`schemas/openapi.yaml`) change in this batch — the SDK version is unchanged._

The biggest update in a while. Rokha's core idea — *find tools, wire them
into a workflow, and run it* — now has real foundations you can touch, much
of it without even signing in.

- **Remix — watch Rokha build, live.** The headline. Press one button and
  Rokha autonomously builds a workflow from the live network in front of you:
  she discovers a real tool and targets it, streamed step by step so you can
  watch her work. It's the clearest answer to "what does Rokha actually do?",
  and it grows as the build-it features grow.
- **Build a workflow — the "Rig" foundation.** Rokha can now assemble tools
  into a flow (a **Rig**), run them, and keep a **trace**: a step-by-step
  record of what each tool did and what came back. As a guest you can
  discover a tool, target it, run it, and read the result — and your
  in-progress work is kept for your session.
- **Your dashboard, your way.** The heads-up display is now a composable
  grid — add, resize, and drag panels to build the view you want, and it
  remembers your layout. Rokha can also bring the most relevant panel into
  focus while you chat; if you'd rather she didn't, you can lock it.
- **See the network breathing.** New live views show what's *fresh*, what's
  *trending*, and who's *building*, plus a visual **Capability Graph** that
  maps how the tools relate to each other.
- **Clearer naming.** What we used to call "Constellations of Work (COWs)"
  are now simply **Rigs** everywhere — the word for a workflow you compose
  from tools.
- **Reliability groundwork.** Plenty of behind-the-scenes plumbing landed to
  support all of the above.

## App — manage your chat history (2026-06-04)

_A frontend/app pass. No SDK or wire-contract (`schemas/openapi.yaml`)
change — nothing here moves the SDK version._

Your conversations with Rokha are yours to manage now. You can **see,
rename, pin, and delete** your past chats — and clear out the ones you
don't need in a single click.

What this means for you:

- **It scales with your plan.** Higher tiers keep more saved
  conversations. When you reach your limit, the oldest **unpinned**
  conversation steps aside automatically — pinned ones are always kept —
  so you never have to babysit the list.
- **Pin what matters.** A pinned conversation won't be auto-retired and
  can't be deleted by accident (unpin it first).
- **Exploring without signing in works better, too.** Your in-progress
  work now sticks with you through the session instead of resetting at
  every step, and starting fresh truly wipes the slate. Abandoned guest
  work is cleaned up automatically after a day.

## Breaking — "COWs" are now "Rigs" (2026-06-01)

_A naming change to the wire contract (`schemas/openapi.yaml`). This is
a **breaking** change, so the schema goes to **4.0.0** and both SDKs to
**0.6.0**. Update your client if you filter or read listing types._

We renamed our composed-workflow concept. What we used to call a **COW**
("Constellation of Work") is now simply a **Rig** — a set of tools and
skills harnessed together into one autonomous workflow. Same idea, a
name that actually fits the rest of the vocabulary (harnesses, rigs).

What this means for you:

- **Listing type renamed.** The `listing_type` value `"cow"` is now
  `"rig"` everywhere it appears — in the Registry, in search filters,
  and in the schema's `listing_type` enum. Code that sends
  `listing_type: "cow"` or branches on it must switch to `"rig"`.
- **SDKs updated in lockstep.** `@rokha/sdk` and `rokha-sdk` (Python)
  both ship `0.6.0` with the new type and a bumped `SCHEMA_VERSION`
  (`4.0.0`). `ro status` and the SDK drift check expect the server to
  serve `4.0.0`.
- **No other shapes changed.** Only the name moved — fields, endpoints,
  and auth are untouched.

## App — works on your phone now (2026-05-31)

_A frontend/app pass making the pre-release Rokha web experience hold
up on phones. No SDK or wire-contract (`schemas/openapi.yaml`) change —
nothing here moves the SDK version._

You can now open Rokha on a phone and actually use it. This pass
focused on the small-screen experience.

- **Built for thumbs.** Chatting with Rokha and reading her live feed
  both work cleanly on a phone now — text wraps instead of running off
  the edge, the menu stays on screen, and the buttons are big enough
  to tap without missing.
- **No more black screen on older phones.** Devices that can't run the
  fancy animated backdrop used to get a blank screen; now they get a
  clean branded backdrop instead, so the first impression always
  lands.
- **Lighter on mobile data.** Heavy parts of the page no longer load
  until you actually open them, so the app feels quicker and uses less
  data on a phone.
- **Polish.** Tidier spacing along the bottom of the screen and a
  cleaner header on the live feed.

## App — one cohesive look + expandable skill cards (2026-05-30)

_A frontend/app design pass on the pre-release Rokha web experience.
No SDK or wire-contract (`schemas/openapi.yaml`) change — nothing here
moves the SDK version._

The whole experience now feels like one product instead of a few
screens stitched together.

- **One calm, consistent look.** Chat, the live dashboard, and the
  Registry now share a single "steel & ice-blue" palette, with small,
  deliberate pops of color (cyan, green, violet) so it has life
  without feeling busy.
- **Skill cards work the same everywhere.** A tool or skill looks and
  behaves identically wherever you meet it — and opening one now
  **expands it in place**: in the Registry the card grows right in the
  grid and nudges its neighbors aside, instead of a small pop-up
  taking over the screen.
- **Easier to follow what Rokha is doing.** When Rokha works through a
  request, her steps read as a clean, scannable flow, and any tools or
  results she pulls back show up as tidy cards — not raw data dumps.
- **Bigger, clearer text** on large screens, and a tidier footer with
  evenly-sized controls.

**Where this gets us:** the experience of discovering tools, talking
to Rokha about them, and watching what she does now reads as one
coherent surface — another step toward the north-star journey
(discover → chat → build → keep it) feeling genuinely good to use.

**What's next:** turning the "build" step on — letting Rokha take a
tool you found and actually start composing something useful with it.

## App — Rokha chat, dashboard & registry polish (2026-05-30)

_A frontend/app update to the pre-release Rokha web experience. No SDK
or wire-contract (`schemas/openapi.yaml`) change — nothing here moves
the SDK version._

A round of UX work on the two halves of the experience: talking to
Rokha, and the live dashboard beside it.

- **Get back to your conversation in one tap.** A show/hide toggle for
  the chat lets you wander off into the registry or another panel and
  then flip straight back to your conversation with Rokha, instead of
  losing your place.
- **A calmer, easier-to-read chat.** The conversation now wears the
  same clean "glass" look as the rest of the dashboard, with clearer
  labels for who's speaking — Rokha or you.
- **Looks right on smaller screens.** On laptops and tablets the
  dashboard now drops the cramped desktop split for a simpler stacked
  layout — while keeping every panel and view you'd get on a big
  monitor. Nothing is removed on smaller screens; it just rearranges.
- **No more spill-over.** Fixed a visual glitch where cards inside the
  dashboard panels could bleed past the panel edge while scrolling.
- **Ask about anything in the registry, instantly.** Tapping a tool or
  skill in the Registry now drops a question about it straight into
  your chat with Rokha — no copy-paste, no losing your place.
- **Smoother, and easier on the eyes.** The animated background is
  lighter on your device, and the dashboard panels rest on a soft
  frosted layer so text stays crisp over the moving starfield behind it.

**Where this gets us:** chatting with Rokha and watching the live
dashboard now read as one coherent surface that holds up from a laptop
to a wide monitor — a real step toward the north-star journey (chat →
build → keep) feeling genuinely good to use, not just functional.

**What's next:** wiring up the remaining dashboard controls, making the
central orb react in real time to what Rokha is doing, and the
production hardening + public CLI/SDK release that gate a public launch.

## 0.5.0 — Federated LLM proxy (slice 1 of LLM routing)

**Additive. Schema bumped 3.1.0 → 3.2.0 (minor — 1 new endpoint). SDK 0.4.0 → 0.5.0.**

Adds typed bindings for the new federated LLM proxy. The proxy lets
any authenticated Rokha caller (CLI, SDK, third-party agent) forward
Anthropic-shaped `/v1/messages` requests through Rokha — using their
own BYO Anthropic key (if stored in their account) or the Rokha
tenant key with the free-tier daily rate limit.

### Added

- **`RokhaClient.llm`** (TS) / **`llm_proxy()` method** (Py):
  - `llm.proxy(body)` → forwards to `POST /api/v1/llm/proxy`. Body
    is the Anthropic messages request; response is Anthropic's
    response shape verbatim.
- TS exports `LlmClient` + `AnthropicMessagesRequest` +
  `AnthropicMessagesResponse` from index.
- `SCHEMA_VERSION` bumped to `3.2.0` in both SDKs.

### Wire contract (schema 3.2.0)

- New tag `llm`.
- New path `POST /api/v1/llm/proxy` with `bearerAuth` security.
- New components: `AnthropicMessagesRequest` + `AnthropicMessagesResponse`
  (intentionally permissive — `additionalProperties: true` — so the
  proxy stays a pass-through as Anthropic's API evolves).

### Server policy (Erebus)

- Caller's user-scoped BYO Anthropic key (via `agent_api_keys`) wins
  when present and bypasses the rate limit.
- Otherwise the tenant fallback is `ANTHROPIC_KEY_ROKHA_AGENT` env
  (then `ANTHROPIC_API_KEY`), and the existing free-tier daily limit
  (`LLM_DAILY_RATE_LIMIT`, default 100) applies — same shape as the
  agent chat routes.
- Only `claude-*` models. Default `claude-haiku-4-5-20251001`.
- `stream=true` is rejected (slice 2 / v0.3.2 adds streaming).

### What this unlocks

- Third-party SDK callers can chat with Anthropic via Rokha (the
  user's account fronts the compute). They never see the key.
- Slice 2 — `rokha-agents` repoint to call the proxy instead of
  Anthropic directly — lands next session. After that, `ro up`
  works end-to-end without `ANTHROPIC_KEY_ROKHA_AGENT` on the
  local machine.

## 0.4.0 — CLI device flow (`ro login`)

**Additive. Schema bumped 3.0.0 → 3.1.0 (minor — 3 new endpoints). SDK 0.3.0 → 0.4.0.**

Adds typed bindings for the CLI device authorization grant (RFC 8628
adapted) that powers `ro login`. Full design:
[docs-internal/src/operations/cli-device-flow.md](../docs-internal/src/operations/cli-device-flow.md).

### Added

- **`RokhaClient.cliAuth`** (TS) / **`cli_auth_*` methods** (Py) — typed
  wrappers around the new endpoints:
  - `cliAuth.start({ scope?, client? })` → `CliDeviceStartResponse`
  - `cliAuth.poll(device_code)` → discriminated `CliDevicePollResponse`
    (`pending` / `slow_down` / `authorized` + jwt + identity / `expired` / `denied`)
  - `cliAuth.authorize(user_code)` — requires bearer JWT; binds the
    code to the caller's identity, mints a 30-day CLI-scoped JWT.
- **`RokhaClient.setAuthToken(jwt)`** (TS) / **`set_auth_token(jwt)`**
  (Py) — sets the Bearer token used on protected routes.
- TS exports `CliAuthClient` + the four response interfaces from index.

### Wire contract

- Schema 3.1.0 adds `/api/auth/cli/{start,poll,authorize}` and four
  components: `CliDeviceStartResponse`, `CliDevicePollResponse`
  (oneOf with `discriminator: status`), `CliDeviceAuthorizeResponse`.
- `SCHEMA_VERSION` bumped to `3.1.0` in both SDKs.

### Notes

- The CLI `ro login` ships in `rokha-cli` 0.3.0 (separate package).
- The Hecate `/cli` browser page is not yet shipped — until it is,
  the user must complete the `authorize` step by `curl`ing the
  endpoint with their existing session JWT. See the design doc.

## 0.3.0 — Schema drift detection wired

**Additive. Schema stays at 3.0.0. SDK 0.2.0 → 0.3.0.**

Closes the gap noted in the 0.2.0 release notes: SDK clients can now
verify they're talking to a compatible Erebus before sending requests.

### Added

- **`RokhaClient.SCHEMA_VERSION`** (TS class static / Py class attr) —
  the schema version this SDK build was compiled against (`3.0.0`).
- **`RokhaClient.checkSchemaCompat()`** / **`check_schema_compat()`** —
  fetches `/api/schema/version` and returns a `SchemaCompatReport`:
  - `match` — server and SDK agree
  - `minor-drift` — same major, server has a newer minor (forward-compatible)
  - `major-drift` — incompatible (SDK should refuse)
  - `unreachable` — schema endpoint did not respond
- TS: `SchemaCompatLevel` and `SchemaCompatReport` exported from index.
- Py: `SchemaCompatReport` dataclass exported from `rokha`.

The SDK does not throw on drift — callers decide whether to warn,
refuse, or proceed.

### Companion: rokha-cli 0.2.0

Same schema-version constant lives in the `ro` CLI (binary renamed
from `rokha` → `ro`). `ro status` runs the same drift check against
the configured Erebus.

## 0.2.0 — Agent rebrand: Hecate/Hex → Rokha Agent

**BREAKING. Schema bumped 2.0.0 → 3.0.0 (major). SDK 0.1.0 → 0.2.0.**

The resident agent is no longer "Hecate"/"Hex" — it is **Rokha**, the
breath that animates the flow. The agent and the framework now share
the name by design.

### Breaking changes

- **Agent routes renamed.** Every `/api/agents/hecate/*` endpoint is
  now `/api/agents/rokha-agent/*` (chat, chat/public, chat/stream,
  chat/stream/public, status, tools, history, clear, model-info,
  available-models). The old `/api/agents/hecate/*` paths are **gone**
  — there are no compatibility aliases. Callers must update.
- **Agent identifier.** The agent's id/dispatch token is now
  `rokha-agent` (was `hecate`). Any code passing `'hecate'` as the
  agent argument must pass `'rokha-agent'`.
- **MCP tool names.** Agent-owned MCP tools exposed at `/mcp/jsonrpc`
  are renamed `hecate_*` → `rokha_agent_*` (e.g. `hecate_remember` →
  `rokha_agent_remember`, `hecate_new_session` →
  `rokha_agent_new_session`). Any MCP client invoking these by name
  must update.
- **Discovery insight field.** `seed_hex` → `seed_rokha_agent` in the
  discovery insight payload.
- **TypeScript:** `AgentsClient.hecateChat()` → `rokhaAgentChat()`;
  all `agent` defaults are now `'rokha-agent'`.
- **Python:** `RokhaClient.agent_status()` / `agent_tools()` (and
  peers) now default `agent="rokha-agent"`.

### Migration

- Replace `/api/agents/hecate/` → `/api/agents/rokha-agent/` and the
  agent token `'hecate'` → `'rokha-agent'`.
- Replace MCP tool names `hecate_*` → `rokha_agent_*`.
- Server-side deployments: run the included DB migrations
  (erebus `011`, harnesses `008`, agents `003`) so seeded keys, rate
  limits, SYSTEM personality, and per-user model choice carry over.

### Notes

- `RokhaClient.SCHEMA_VERSION` / `nb.checkSchemaCompat()` are not yet
  implemented in this SDK (pre-existing gap, unrelated to this
  release); the canonical schema version is `info.version: 3.0.0` in
  `schemas/openapi.yaml`, served at `/api/schema/version`.
