# Feedback & bug reports — how to reach us

Rokha is in **V1 Beta**. Things will break, and the fastest way to get them
fixed is to tell us precisely. There is no wrong door — pick whichever is least
friction for you.

---

## The three doors

| You are… | Use | Needs a GitHub account? |
|----------|-----|-------------------------|
| **Using rokha.ai** and something broke | The **🐞 Report a bug** link in the site footer | **No** |
| **Building on the SDK / CLI / API** | [Open an issue on `rokha-sdk`](https://github.com/aetherBytes/rokha-sdk/issues/new) | Yes |
| Reporting something **sensitive** (a security issue, private data) | Email — see [Security](#security-issues-do-not-file-publicly) | No |

### 1. The in-app form (easiest — no account needed)

On **[rokha.ai](https://rokha.ai)**, click **🐞 Report a bug** in the footer.
Write what happened, hit send. That's it.

We file the report into our issue tracker **for you**, so you don't need a
GitHub account and you never leave the page. You get a **reference number**
(like `ROKHA-123`) — quote it if you follow up with us.

**What the form sends along with your words:** the page you were on, your
browser, the screen size, the build you're running, and whether you were signed
in. That's it. **No IP address, no wallet, no keys, no session token.** You can
see the full list in the form itself before you send.

Leaving a contact (email or an @handle) is **optional** — if you skip it we
simply can't reply, but the report still lands.

### 2. GitHub issues (for developers)

If you're integrating the SDK, the CLI, or the API, file it where the code is:

**→ [github.com/aetherBytes/rokha-sdk/issues](https://github.com/aetherBytes/rokha-sdk/issues)**

This is the right door when you want a thread you can subscribe to, paste code
into, and follow to a fix. We triage from there.

---

## What makes a report we can actually fix

A good report answers three questions. You don't need all three — but each one
you can give roughly halves the time to a fix.

1. **What did you do?** The steps, in order. "I opened the News tab on my
   iPhone" beats "the menu is broken."
2. **What did you expect, and what happened instead?** The gap between those two
   is the bug.
3. **Can you make it happen again?** Every time / sometimes / it happened once.
   "Only after I log in" is a huge clue.

Extras that help a lot when you have them:

- **The exact page or URL** (e.g. `rokha.ai/@sage/hoodwatch-audit`).
- **A screenshot** — worth a thousand words on anything visual.
- **A run's trace** if a workflow misbehaved: every run keeps a receipt of what
  went in and what came out, and the trace usually contains the answer.
- **The error text**, verbatim, if you saw one.

A template you can paste:

```
What I did:
What I expected:
What happened instead:
Happens every time? yes / sometimes / once
Page:
```

## What to send where

- **The site misbehaved** (a button, a layout, a page that won't load) → the
  in-app form. It attaches the page and build automatically.
- **A workflow ran and the result was wrong** → the in-app form, and mention the
  workflow and the input you gave it.
- **The SDK, CLI, or an API response is wrong** → GitHub issues on
  [`rokha-sdk`](https://github.com/aetherBytes/rokha-sdk/issues), with the
  request you sent and the response you got.
- **A published skill or MCP server is broken/malicious** → tell us via either
  door and name the listing. We can delist.
- **You have an idea, not a bug** → both doors take ideas. The in-app form has a
  "✦ I have an idea" mode.

## Security issues — do NOT file publicly

If you've found something that could expose data, let someone act as another
user, escape the sandbox, or spend someone else's balance: **do not open a public
issue**, and please don't post it on social.

Email **contact@rokha.ai** with the details and give us a chance to fix it first.
We'll confirm we got it, keep you posted on the fix, and credit you if you'd like
the credit. Responsible disclosure is genuinely appreciated — and it's the one
category where a public report does real harm to real users.

## What happens after you report

1. It lands in our tracker, labelled and with your diagnostics attached.
2. We triage — reproducible bugs move first, and anything touching security,
   money, or data goes to the front of the queue.
3. If you left a contact, we'll reach out — especially if we can't reproduce it
   and need one more detail from you.
4. Fixes ship continuously; there's no release train to wait for.

## Watching the work

- **Public changelog** — [`CHANGELOG.md`](../../CHANGELOG.md) is written in plain
  English: what changed, where it gets us, what's next.
- **Status of the SDK/API contract** — the wire contract is published at
  `/api/schema` and its version at `/api/schema/version`, so you can always tell
  what your client is talking to.
- **X** — [@Rokha_ai](https://x.com/Rokha_ai) for what's shipping.

---

**Thank you, genuinely.** A beta is only as good as the reports it gets, and the
person who takes two minutes to describe a broken menu is doing real work for
everyone who comes after them.
