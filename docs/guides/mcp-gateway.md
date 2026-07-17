# Your MCP Gateway

Bundle any number of external MCP servers behind **one** Rokha endpoint. Point
any MCP client — Claude Code, an IDE, your own agent — at that single URL with
your Rokha token, and every server's tools appear, namespaced, with credentials
injected for you. **The upstream keys never leave Rokha.**

## Why

Running many MCP servers means many URLs, many tokens, and secrets copied into
every client. The gateway collapses that to one URL + your Rokha token. Rokha
holds each server's credential (a saved key or a Connect alias) and injects it
server-side at call time, so the client that talks to your gateway never sees an
upstream secret.

## Add a server

In Rokha: **Profile → API Keys → Your MCP gateway**:

1. **name** — a short slug (becomes the tool namespace, e.g. `github`).
2. **`https://server/mcp`** — the server's MCP endpoint URL.
3. **alias (optional)** — a saved key or Connect alias that authenticates this
   server. Leave blank for open (unauthenticated) servers.

Server URLs are SSRF-validated. Up to **20 servers** per account.

## Point a client at it

Your **gateway URL** shows above the server list once you've added one — it
looks like `https://rokha.ai/api/gateway/mcp`. It speaks MCP over JSON-RPC; the
bearer is **your Rokha token**.

Claude Code example (`.mcp.json`):

```json
{
  "mcpServers": {
    "rokha-gateway": {
      "type": "http",
      "url": "https://rokha.ai/api/gateway/mcp",
      "headers": { "Authorization": "Bearer <YOUR_ROKHA_TOKEN>" }
    }
  }
}
```

## How calls flow

- **`tools/list`** — the gateway fans out to every enabled server you added and
  returns their tools, each namespaced `<server>_<tool>` (so `search` on your
  `github` server is `github_search`). Name collisions across servers are
  impossible.
- **`tools/call`** — the gateway de-namespaces the tool, resolves the server's
  vault alias (refreshing an OAuth Connect token just-in-time when needed),
  injects the credential, and forwards the call. The response comes straight
  back to your client.

## Security posture

- **Keys stay in Rokha.** The client authenticates to the *gateway* with your
  Rokha token only; upstream credentials are injected server-side and are never
  returned to the client.
- **Per-owner isolation.** A gateway only ever exposes the servers *you*
  registered, resolved against *your* vault.
- **SSRF-guarded** server URLs; a hard cap of 20 servers per account.

## Related

- **Connect** — connect an account once (GitHub, Google, Slack, …) and
  reference it as an alias here, so the gateway injects a live, auto-refreshed
  token.
- **Saved keys** — store any raw API key as a custom secret and use its alias
  as a server's credential.
