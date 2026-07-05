# Changelog

All notable changes to the Rokha SDK — and notable updates to the
Rokha product it talks to — are documented here. The SDK is the public
face of Rokha; the wire contract it depends on is
`schemas/openapi.yaml`, served live at `/api/schema`.

## Plans that tell the truth — and a payments rail under them (unreleased)

The subscription surface grew up. Every claim on the Plans page is now a
number the platform actually enforces, and real payments are wired and in
test.

- **Honest plans, plainly stated.** Every plan now includes the whole
  product — discovery, building, real runs, secrets, scheduling,
  publishing. Paid tiers raise your daily capacity (AI fuel, sandbox runs,
  saved chats); they don't unlock hidden features. The Plans page states
  exactly what each tier gets, and every number matches what the platform
  enforces.
- **See your allowance live.** The Plans page now shows live meters for
  every daily limit — how much AI fuel and how many sandbox runs you've
  used today, when they reset — read from the same counters the platform
  enforces with, so they can't drift.
- **Pro is coming soon.** The top tier is visible but not yet purchasable —
  we're sizing it from real usage before opening it.
- **Real payments, safely.** Subscriptions flow through a hosted checkout
  by a major payment processor — your card details never touch Rokha's
  servers, and cancellation stops billing on both sides. Currently in test
  mode ahead of launch.
- **Everyone plays by the same rules.** Operator accounts no longer get any
  special capacity — the same limits, budgets, and model access apply to
  every account. Admin access means the admin console, nothing more.
- **Pick a model per step.** A workflow block can now pin the AI model it
  runs on — "preferred" swaps down gracefully when the pin isn't available;
  "required" refuses to run without it, with an honest typed error. Proven
  end-to-end: one workflow ran its first step on a fast model and its
  second on a bigger one, with each step's model recorded in the run's
  receipt.
- **See your allowance before you sign in.** The live allowance meters are
  also a side-rail view now — anonymous visitors see the limits tied to
  their browser session (AI fuel, runs, chats, and when they reset), read
  from the same counters the platform enforces with. Checking your usage
  never spends any of it.
- **Outside agents get the same powers.** Any agent building workflows over
  Rokha's public MCP endpoint can set per-step model pins too — the tool
  descriptions teach the semantics, and the audit fixed a bug where
  workflow steps authored by outside agents could be silently skipped at
  run time. Two front doors, one behavior.

## Your account grows up: profile, private keys, and an assistant who can run the whole show (unreleased)

The first big pass on the logged-in experience — plus a set of upgrades that
make Rokha (the assistant) genuinely able to guide you from first question to
real execution.

- **A real profile.** Upload a profile photo, tune how Rokha addresses and
  helps you with quick "personality notes" (she reads them on every reply),
  browse everything she remembers for you with clear filters — and see at a
  glance which memories shape her behavior versus plain account records.
- **Bring any key, never expose it.** The account vault now stores ANY named
  secret (a GitHub token, a weather key…), not just a model key. Attach a
  saved key to a workflow block by its short name and the value is filled in
  server-side only at the moment of the live call — it never appears in your
  workflow, your receipts, or the assistant's view. Masked forever after
  saving; nothing can echo it back.
- **Your sessions, back in your hands.** Saved conversations now have a real
  Open button — jump back into any chat from your profile. This pass also
  fixed a real bug where non-wallet accounts never saw their history at all.
- **Your work on the public pulse — anonymously.** Activity you set to
  "pulse" now genuinely shows in the public live streams as an anonymous
  operator handle, with private content stripped. Private stays private.
- **Sign-in, simplified for launch.** Rokha will launch with Google and
  crypto-wallet sign-in only — we don't store passwords, by design. (Email +
  password stays built but switched off.)
- **Publish safely.** Publishing to the registry now hard-refuses anything
  that carries account data — profile records, chat history, or attached
  keys can never end up in a public listing.
- **Ready-made workflows stay light.** The starter Explore/Audit workflows
  now read a tool's real listing and document instead of trying to execute
  it — so they run instantly for ANY tool, no runtime needed. Heavier
  workflows that truly execute tools build on top, via a live connection or
  the cloud sandbox.
- **The assistant runs the sandbox now.** Ask for a sandbox and Rokha can
  start it (it costs a run from your daily allowance — she says so and asks
  first), check on it, run work inside it, and shut it down when you're done.
- **She can walk you there.** Ask Rokha to "show me the builder" or "open the
  registry" and the app actually takes you to that surface, with the relevant
  panel highlighted — and a long-dead piece of her guidance (pointing at your
  workflow while you build) is back alive.
- **Published creations are first-class listings.** Your published skills and
  workflows now show their full document, and the catalog can be filtered to
  Rokha-published entries like any other source.

## A friendlier front door, a livelier Rokha, and a tougher directory (unreleased)

A large polish-and-hardening pass across the whole pre-login product, shaped by
a founder walkthrough playing a brand-new visitor. Highlights:

- **Three doors on the front page.** The landing now shows you the whole product
  at a glance: *Chat with Rokha* (ask in plain words), *Explore the Registry*
  (130k+ skills & live servers from 9 registries), and *Build & run for real*
  (compose a workflow — every run leaves a receipt) — plus the standing promise,
  stated plainly: test any tool in an isolated sandbox **before** it ever
  touches your own agents.
- **Rokha feels alive now.** Send a question and seeds gust in toward the mark;
  while she works, a calm glowing orbit circles the logo; and when she finishes,
  a whirlwind of seeds spirals up across the screen. Subtle, fast, and switched
  off automatically for reduced-motion users.
- **Connecting to modern live servers actually works.** Two real bugs were found
  and fixed by exercising live connections end-to-end: many current tool servers
  rejected our handshake outright, and servers that require a session dropped
  the follow-up calls. Both fixed — discovery *and* real execution now work
  across the modern server ecosystem, including servers exposing 80+ tools.
- **A tougher, more honest directory.** The registry now probes live servers for
  real (dead links get flagged with evidence, not hidden), shows each server's
  actual tool list on its page, displays licenses everywhere, adds ~20 browse
  categories and a "moving now" strip, and serves crawlable listing pages so
  the catalog can be found from a plain web search.
- **Building without fear.** A new "builds in progress" indicator in the navbar
  means navigating away never loses your work — one click jumps you back into
  the skill, harness, or rig you were making. Blocks in a workflow can be
  dragged to reorder. Hand-off buttons (turn a harness into a rig) now *add to*
  what you were building instead of replacing it.
- **Smarter ready-made workflows.** Start a template from a specific listing and
  that listing becomes step one — configurable like any other block. Start from
  just a name and the registry search stays as step one, with the found tool
  landing as its own editable block right after.
- **Better on phones.** The builder got solid, readable surfaces and a trimmed
  layout on small screens — one idea per line, everything still reachable.
- **The sandbox panel grew up.** One big obvious Start button, clear
  run/restart/stop controls, and a click-to-load command helper with ready-made
  templates for testing APIs and probing live tool servers by hand.

## Your own cloud sandbox — persistent, browser-capable, and honest (unreleased)

- **A sandbox that stays with you.** Start an isolated cloud machine once and it
  stays up for your whole session — a real shell (curl, npm, python, git) plus
  live Model Context Protocol connectivity. Watch every command and result in
  the new SANDBOX panel; it winds down on its own when idle.
- **The assistant drives it too.** Ask Rokha to "curl this url in my sandbox"
  or "list that server's tools" and the command runs for real in YOUR sandbox,
  with the receipt in the log.
- **Live servers are now runnable.** Any listing with a live connection can be
  worked directly: the runtime connects to the server, discovers its real
  tools, and uses them — no install.
- **Attach catalog servers by name.** Tell the sandbox to attach a known tool
  server (say, a browser-automation server) and it resolves the right package
  from the registry, starts it inside your sandbox, and keeps it warm for the
  session — repeat calls answer in seconds.
- **A real browser, when you need one.** Start a *browser sandbox* and page
  automation works for real: navigate, read, click — the actual page, not a
  summary of one. Browser sandboxes use a bigger machine, so they draw a bit
  more of the day's free allowance (shown plainly before you start).
- **Run it for real, or it says no.** The old "demonstration of what this tool
  would output" is gone everywhere. If something can't genuinely execute, Rokha
  tells you exactly what's missing and where the real run lives — she never
  produces pretend output. Every run's receipt now also records precisely what
  input the step consumed.

## v0.1.0 — the first versioned cut (2026-07-03)

Rokha's public surface graduates from the pre-release `0.0.0-dev.1` freeze to
**v0.1.0** — one number across the wire contract (`/api/schema`), the
TypeScript and Python SDKs, and the `ro` CLI. `v1.0.0` lands when the
logged-in experience reaches the same bar as the pre-login product.

What's live behind this cut:

- **One registry, nine sources.** The catalog now folds in the Official MCP
  Registry, Glama, skills.sh, Docker's MCP Catalog, and Anthropic's own Agent
  Skills alongside ClawHub and Smithery — ~131,000 findable skills and
  servers, each with the richest details its home registry exposes (install
  counts, pull counts, quality scores, licenses, live endpoints).
- **The cloud runtime runs more of the catalog.** Real sandboxed execution now
  works for skills from ClawHub, skills.sh, Anthropic's skills, and Rokha's
  own — not just one source. Every run still produces a full receipt.
- **More free runs while we're pre-launch.** Visitors get 5 free real runs a
  day (was 1); free accounts get 25 (was 5). Platform-wide daily ceilings
  still bound total spend.

## Real lookups, fuller receipts, and a front page that says what you get (2026-07-03)

- **Starter workflows do a real search.** The "Explore" and "Audit" starters'
  first step now performs an actual registry search and fetches the listing's
  real document at run time — the analysis downstream works from the genuine
  artifact, never a best guess.
- **Receipts record what actually flowed in.** Every step's receipt now
  separates *what came in* (the previous step's full output, or the workflow's
  own input) from *how the step was configured* — including runs in the cloud
  sandbox. Reading a receipt now answers "what did this step actually consume?"
  at a glance.
- **The whole final answer, in chat.** A workflow's last step is its
  deliverable — the assistant now shows it in full instead of trimming it
  mid-sentence (with an honest pointer to the receipts if it's truly huge).
- **The registry front page states the offer.** Live listing count (54k+ and
  growing), what every listing carries (run signals, quality scores, what it
  needs), and the part agents care about: the same registry is readable through
  one standard MCP endpoint — Claude Code, Cursor, or any MCP client. The
  builder's intro now also says the quiet part: everything you make is a
  standard, portable SKILL.md.
- **Security hardening.** Tightened ownership checks on the agent door
  (private memory can only be edited or deleted by its owner) and fixed scope
  handling for email/Google accounts. Found in an internal review; no known
  exploitation — we're pre-launch.

## Build workflows like building blocks — and any agent can, too (2026-07-02)

Following the V1 polish, a round of building + discovery refinements:

- **One skill card, everywhere.** Whatever surface you meet a skill on — the
  registry, the assistant's live feed, a lookup while you build — it opens
  into the exact same detailed card: what it does, how it runs, whether it's
  verified, what it needs, and the actions to try or add it.
- **Look things up without losing your place.** Open a skill's details and it
  appears in the side panel right where you're working — you never get yanked
  to another screen. Ask the assistant to "find a tool for X" and the relevant
  panels quietly filter to the answer and light up to let you know, instead of
  hijacking your view. A new **Skill Stats** readout shows reach, run-signal,
  trust, and scope for whatever skill you're focused on.
- **Chain blocks like a workflow.** Add a block **above or below any block**,
  not just at the end. Each block can optionally declare what it **expects**
  from the step before it and what it **produces** for the next — so a chain
  of tools and instructions reads clearly end to end (the steps already pass
  their output forward automatically; this just makes it legible). After a run,
  the line between two blocks shows **how long that step took** and clicks
  through to its receipt. The composer can pop to **full screen** when a step
  gets involved.
- **A sharper starter.** The "Explore" starter workflow now begins by asking
  what tool you want to dig into, then searches the Rokha registry for it —
  a clean, obvious first step before you run.
- **Agents get the full toolbox.** Outside agents connecting over the open
  tool interface can now do everything a person can: search + read the
  registry, author skills/harnesses/rigs, keep private memory, run for real,
  and publish to the registry — with their identity handled safely for them
  (they never pass or spoof a scope). Publishing works the same from the app
  or from an agent.

## The V1 polish wave — one story, one panel, chaining for everyone (2026-07-01)

A top-to-bottom pass through the eyes of a first-time visitor, with real
capacity guarantees underneath:

- **One story everywhere.** The landing, the chat intro, the registry, and the
  builder now tell the same story in the same words: Rokha is your librarian —
  ask in plain words, she finds a real tool, runs it live, and shows you the
  receipt. One vocabulary for how a skill runs ("Runs now" / "Runs in the
  cloud" / "Live connection") across every card, chip, and filter.
- **One assist panel.** Everything that helps while you work — chat, run
  receipts, Rokha's live reasoning, registry lookups, the doc you're
  authoring, live network readouts — lives in one side panel. Open extra
  containers, split them either direction, drag to move, resize. New info on
  a tab you're not watching pulses its chip; nothing ever yanks your view.
- **Chaining is free.** Multi-step workflows no longer wait for an account —
  chain as many steps as the flow needs, signed in or not. Accounts keep the
  keeps: a saved library, publishing, composing workflows into bigger ones,
  scheduled runs.
- **Fair-use budgets, honestly enforced.** Free usage is bounded server-side
  by daily token budgets — per person and platform-wide — so the free tier
  stays healthy. Hit one and the app says so plainly (it resets tomorrow;
  your own API key or a plan raises it). Runs also carry per-run ceilings
  sized to your tier.
- **Trust on every listing.** A 🛡 safety-read button asks Rokha to vet the
  exact listing (what it accesses, what could go wrong); verified flags and
  quality scores now show where they exist; a requirements row says what a
  skill needs to run; and an unclassified listing says so instead of showing
  nothing.
- **Workflows are files.** A rig now resolves to a portable, standard skill
  file — the whole flow, its inputs and outputs, and how to run it via
  Rokha's open MCP/API doors — so any connected agent can read it and run
  the workflow like a skill. A publish button lives right in the builder.
- **Watch a live build.** The one-button demo, the guided tours, and the
  starter prompts were all re-checked against the app as it is today — every
  button does what it says, and the tours drive the new panel for real.

## Compose: a rig can now run another rig (2026-07-01)

The last layer of the creations story — composition:

- **Add a whole rig as a step.** Signed in, the rig builder's add-row gains
  **＋ rig**: pick any of your saved rigs and it becomes a step in the one
  you're building. At run time the inner rig runs whole — its final output
  flows to the next step, exactly like any other step's result.
- **Live reference, not a copy.** The step points at your saved rig, so
  improving the inner rig improves every rig that uses it.
- **One level deep, honestly enforced.** A rig-in-a-rig-in-a-rig (or a loop of
  rigs) politely refuses with a clear message — composition runs one level
  deep for now, and the run record says so.
- **Traces nest.** The inner rig's steps record under the same run, so the
  full story of a composed run reads in one place.
- **Shared harnesses arrive configured.** Adding a published harness from the
  registry now carries its full configuration into your rig — endpoint, tool,
  arguments, instruction — instead of landing blank.
- Agents compose through the same doors (the harness/rig tools accept rig
  references), so an assistant can build a composed workflow end to end.

That completes the signed-in arc: build → keep → carry over on login →
publish → compose. Next: polish and opening these doors on the live site.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Your creations, kept and shared — sign-in work carries over, and publishing arrives (2026-07-01)

The signed-in story now goes end to end:

- **What you build before signing in comes with you.** Log in and the rig,
  harness, and chat history you built as a guest transfer into your account
  automatically — nothing to redo, no lost work.
- **A real "My Creations" library.** The build hub now lists everything you've
  made — skills, harnesses, and rigs — with one-tap open-in-its-builder,
  publish, and delete.
- **Publish to the registry.** A creation can go live as a real registry
  listing, searchable by anyone alongside the tens of thousands of synced
  skills. Publishing the same name again updates it; names are first-come per
  user; payloads that look like they contain keys or tokens are rejected; and
  listings show your display name, never your wallet. Unpublish any time.
- **Agents publish through the same door.** A new signed-in `registry_publish`
  tool (documented in the API schema, with `POST /api/marketplace/registry/publish`,
  unpublish, and a "my listings" endpoint) lets the assistant — or your own
  agent — save and publish on your behalf.

Next on this track: composing your saved pieces into bigger workflows — a rig
that uses another rig as a step.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Build a Rig on your phone — and your creations start to stick (2026-07-01)

Two big steps toward the full builder experience:

- **The Rig builder now works on phones.** The whole make-things path — write a
  skill → wrap it in a harness → turn it into a Rig → run it — no longer
  dead-ends on mobile. The simple linear flow builds and runs on a phone;
  the advanced layouts (graph, loops, trees) politely point you to a bigger
  screen. The builder also got a calmer look: one structure picker instead of
  a row of four, clearer buttons, and styling that matches the rest of the app.
- **Signed-in creations now save for real.** Build a skill or a harness while
  signed in and hit Save — it's kept in your account (saving the same name
  again updates it, no duplicates). The assistant can save on your behalf too,
  through a new signed-in `skill_save` tool — and it now *sees* the draft you
  have open, so "what's missing from my draft?" gets a real answer.
- Signed-out building is unchanged: build and test everything free; saving,
  libraries, and publishing are what signing in unlocks.

Coming next on this track: your pre-sign-in work carrying over when you log in,
a "My Creations" library, publishing to the registry, and composing your saved
pieces into bigger workflows.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Agents can build harnesses too — and a smoother build path (2026-07-01)

Following the human "Build a Harness" workbench, an agent can now do the same
thing over the API — two doors to the same place.

- **Configure a harness from code.** A new authoring tool lets any agent set up
  a harness — a skill (or a plain instruction) wired with its instruction,
  optional label, and, when it's a live tool, an endpoint and its arguments —
  and get the finished configuration back. In the Rokha app the very same tool
  fills the human's live builder form, so a person and an agent build the exact
  same way. It's free to use before signing in, just like building by hand.
- **A clear path from idea to workflow.** The builder now links the pieces
  end to end: turn a skill you just wrote into a harness in one tap, then turn
  that harness into a runnable Rig — no copying, no dead ends.
- **The assistant sees what you're looking at.** When you ask about a specific
  item from the library, the assistant now knows exactly which one is on your
  screen, so it answers about *that* one instead of guessing.

## Build a Harness — configure a skill and see it as a portable file (2026-07-01)

The "Build a Harness" area is now a real workbench. A harness is a capability
that's been *configured and made ready to run* — either a skill from the library
wired up to run, or a plain instruction you hand to the assistant.

- **Start from a choice, not a wall of options.** Pick "wrap a skill" or "write
  an instruction," then get just the form for that path.
- **Find a skill fast.** The skill finder opens on popular skills right away and
  gives you one-click filters — most downloaded, latest, top rated, an
  execution-type filter (runs instantly vs. runs in the cloud vs. a live
  connection), and real tag chips pulled from the results. Filters now work
  together with search and page through the full catalog correctly.
- **Inspect before you commit.** Clicking a skill opens the same rich detail
  card used elsewhere — clearer, easier-to-read text, a labeled description, and
  a one-tap "Add to Rig."
- **A harness is a portable file too.** As you build, a side panel shows your
  harness written out as a standard skill file — the same open format skills
  use — so it's inspectable and copyable. These configured files note that they
  need the Rokha runtime to run.
- **Your work sticks around.** Refreshing or switching tabs keeps you on the
  page you were building, with your progress intact.

Under the hood, the registry listing API gained an `execution_class` filter so
tools can narrow the catalog by how a capability runs.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Easier to start, consistent on every screen (2026-07-01)

More newcomer-friendly polish across the app:

- **A gentler way to begin building.** Before the full set of build options,
  there's now a calm first step that explains, in plain words, what "building"
  even means here — then a single "Start building" button to dive in. It greets
  first-timers and steps aside for people who already know their way around
  (and it remembers where you left off, so you're never bounced back to the
  start).
- **One consistent look.** Section titles and backgrounds now match across the
  app, so moving between the chat, the registry, and the build area feels like
  one place rather than three.
- **Cleaner on phones.** The first-run screens and the registry's quick filters
  are tidier and easier to tap on small screens — contained, evenly sized, and
  no more awkward full-bleed backgrounds.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Meet Rokha — a friendlier first run (2026-06-30)

Opening Rokha for the first time now greets you instead of dropping you onto a
blank screen:

- **A plain-English intro.** Rokha is your **Librarian** for the agentic world —
  the helper who finds a real capability, runs it for real (no install, no
  setup), and shows you the trace to prove it. The first view says exactly that.
- **Starter prompts.** A few one-tap "try asking…" suggestions so you have
  somewhere to begin, plus a simple three-step "how it works."
- **A clear nudge to the chat box** so it's obvious where to type, and a
  "what is Rokha?" explainer one click away.
- **Polished on phones.** The same first-run guidance now works on mobile, the
  intro reads cleanly on a small screen, and you can add things to your rig
  right from the listings.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## A calmer, clearer way to browse the registry (2026-06-30)

The registry — where you discover capabilities to build on — got a
newcomer-first facelift:

- **Opens calm, not crowded.** Instead of a wall of filters and cards, the
  registry now greets you with a plain-English intro to what a skill is, a
  prominent search, and a short "popular right now" list. The full catalog,
  with all its filters, is one search or one click away.
- **One clean reading column.** Browsing the full catalog is now a single
  scrollable column (no more grid-vs-list toggle to fuss with). Every listing
  is clearly tagged by what it is — **Skill**, **Harness**, or **Rig** — and by
  how it runs, so you can tell at a glance what you're looking at.
- **Fuller descriptions.** Some registries only hand back a short blurb in
  their listings; where a fuller description is available, Rokha now shows it
  so you get the whole picture before you commit.
- **Read here, act there.** Opening a listing shows what it does and ideas for
  what you can build with it; the actions — add it to your rig, start building,
  view its source — sit together in one place.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Skill authoring goes full-spec — declare runtime needs (2026-06-27)

The `skill_author` tool (and the in-browser builder) now support the complete
[agentskills.io](https://agentskills.io/specification) frontmatter, so an
authored skill can declare everything the spec allows — including whether it
needs a runtime:

- **`compatibility`** (≤500 chars) — the spec's human-readable signal that a
  skill needs more than plain instructions: system packages, a language
  version, network access, or bundled scripts (e.g. `Requires Python 3.14+ and
  uv`). Agents that read it can warn, check the environment, or decide whether
  to activate the skill. Omit it for pure-instruction skills.
- **`metadata`** — an arbitrary string→string map for extra properties not
  defined by the spec (e.g. `author`, `version`).
- **`allowed_tools`** is now correctly **space-separated** with optional
  scoping, matching the spec (e.g. `Bash(git:*) Bash(jq:*) Read`), and the
  builder nudges least-privilege scopes.

`skill_author` over `/mcp/jsonrpc` accepts the new optional `compatibility` and
`metadata` arguments and returns them in the assembled SKILL.md. Additive and
backward-compatible — existing callers are unaffected.

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Build a skill, run it anywhere (2026-06-25)

You can now build a portable Agent Skill — a standard SKILL.md — right in
Rokha, and any agent can author one the same way:

- **In the browser:** a guided builder walks you through a skill's name,
  its description (what it does + when to use it), and its instructions,
  with a live preview of the SKILL.md as you type and one-click Pretty /
  Raw / Copy. Build and test for free; saving and publishing arrive with
  an account.
- **Test it with Rokha** before you save — she runs your skill on a real
  example and tells you what to tighten.
- **For agents (MCP):** a new public `skill_author` tool builds a
  standards-compliant SKILL.md from `name` + `description` +
  `instructions` (optional `allowed_tools` / `license`) over
  `/mcp/jsonrpc` — no account required. The same tool fills the human
  builder's form, so people and agents author skills the same way
  (two-front-door parity).

_No version bump — Rokha is still pre-release (`0.0.0-dev.1`)._

## Honest pre-release versioning (2026-06-16)

Rokha is still pre-release — no public launch, no users yet — so the
version numbers now say so. Everything (the SDK, the CLI, and the wire
contract) resets to a development version, `0.0.0-dev.1`. The earlier
numbers (schema 4.8.0, SDK 0.8.0) implied a maturity we haven't claimed
yet. We'll publish a real `1.0.0` when we're out of pre-release.

Also in this pass, an honesty fix to the public contract: it no longer
advertises a couple of internal details it didn't need to expose (an
email-verification token field, raw per-token model pricing, and listing
authors' wallet addresses).

## Product — a faster, cleaner catalog (2026-06-13)

_No SDK or schema changes — a performance + design release._

The skill catalog got quicker and tidier:

- **Search is dramatically faster.** Looking through tens of thousands of
  skills now returns in a blink instead of a noticeable pause.
- **Filters are instant and full.** Filtering by how a skill runs used to
  crawl and often showed only a handful of cards; now it returns a full
  page right away, and the catalog remembers how each skill runs so the
  filter keeps getting faster for everyone.
- **Browse at your own pace.** Results load in clean pages with a "Load
  more" button instead of an endless scroll, and narrow filters
  automatically fill the screen so you're never left staring at three
  results.
- **A real loading screen.** The Rokha mark now greets you while a view
  loads, instead of a flicker.
- **Phone polish.** The catalog's filter controls and the view picker
  were reworked to fit small screens cleanly edge-to-edge.

## Product — a friendlier front door (2026-06-12)

_No SDK or schema changes — a design release._

The whole experience got a usability overhaul, phones first:

- **The landing now says what Rokha is.** "Pick a skill. Watch it run." —
  with one button to try a skill and one to watch Rokha build a workflow
  live.
- **Simpler first view, growing as you do.** The editor opens as one
  clean workbench; extra panels appear when they're useful instead of
  all at once. Every view explains itself in plain words.
- **Real phone support.** Bottom tab navigation, readable text sizes,
  full-width skill descriptions, one-tap filters, and a labeled menu for
  the rare controls.
- **Find skills by how they run.** A new filter splits the catalog into
  skills that run instantly, skills that use the cloud runtime, and live
  API/MCP servers.
- **Smoother flow.** After adding a skill to your workflow, one tap takes
  you straight to building it.

## 4.8.0 / SDK 0.8.0 — agents can now browse and adopt skills, no account needed (2026-06-11)

Any AI agent that speaks MCP can now use Rokha's registry directly:

- **Public discovery.** The MCP endpoint's handshake (`initialize`,
  `tools/list`) needs no authentication — an outside agent can see the
  toolbox before deciding anything.
- **Two new public tools.** `registry_search` searches 30,000+ published
  agent skills by plain text; `registry_get_skill` returns an
  install-ready SKILL.md (standard agent-skills format, frontmatter
  included) plus what the skill needs to run — everything an agent needs
  to add the skill to its own library.
- Proven end-to-end with Claude Code as the test agent: it searched the
  registry, fetched a design skill, and installed it into its own skill
  list — over the public MCP door, zero setup.
- Account-scoped tools (memory, tasks, agent messaging) still require
  authentication, as before.

- **New first-party skill: `rokha-registry`.** A portable agent skill
  (in this repo's `skills/`) that teaches ANY agent the flow above —
  search, vet, install — so the capability travels with the skill file
  itself. Vetting downloads before installing is the documented practice.

SDKs `0.8.0` (TS + Python) track schema `4.8.0` (additive).

## Product — Run for real: skills actually run now (2026-06-11)

_The headline. No SDK or schema changes — a new runtime._

The thing the whole platform was built for: **skills now actually run.**
Open a skill that needs real tooling, hit **⚡ Run for real**, and Rokha
spins up an isolated cloud sandbox, installs and executes the skill for
real, and hands you back the result with a full record of every command
it ran — no install on your machine, no setup, no API keys. The recipes
finally have a kitchen.

- **One free real run, no login.** Every visitor gets a genuine sandboxed
  execution to try it. Logged in unlocks more.
- **Honest by design.** What runs is real; the receipt proves it. A skill
  that needs a tool the runtime doesn't carry yet says so plainly instead
  of faking a result.
- **Safe + bounded.** Every run is isolated, time-limited, and holds no
  secrets; the free runtime has a daily budget so it stays sustainable.

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
