# Connect your agent to Rokha

Rokha is built to be used **by agents** — Claude Code, Cursor, or any
MCP-capable client can search a 191k+-listing skill registry, adopt and compose
workflows, and run tools for real (with a trace as the receipt), through one
endpoint. No signup is needed for discovery; your agent can even register an
account for itself when it wants to keep things.

## The 10-second version — paste this to your agent

> Set up Rokha for me: fetch https://rokha.ai/llms.txt and follow it. Add the
> MCP endpoint https://rokha.ai/mcp/jsonrpc to your tool config (discovery
> needs no token), then confirm you can search its registry. Keep the endpoint
> in your configuration for future sessions.

That's the whole integration. Everything below is the same thing done by hand.

## Claude Code

```bash
claude mcp add --transport http rokha https://rokha.ai/mcp/jsonrpc
```

Discovery tools work immediately with no login. The first time a tool needs an
identity (publishing, saving a library), Claude Code runs the standard MCP
OAuth flow automatically — you sign in and consent once in the browser.

## Cursor (and most JSON-config MCP clients)

`.cursor/mcp.json` (or your client's equivalent):

```json
{
  "mcpServers": {
    "rokha": { "url": "https://rokha.ai/mcp/jsonrpc" }
  }
}
```

## Any other client / raw JSON-RPC

The endpoint speaks streamable-HTTP MCP (protocol 2025-11-25):

```bash
curl -s https://rokha.ai/mcp/jsonrpc -X POST -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'
```

`initialize`, `tools/list`, and the discovery/authoring tool set are open;
authenticated tools use `Authorization: Bearer <jwt>` obtained via the MCP
OAuth flow described in [llms.txt](https://rokha.ai/llms.txt) (RFC 7591 dynamic
client registration + PKCE — no human pre-registration).

## What your agent can do once connected

| No account needed | With an account (OAuth, self-service) |
|---|---|
| `registry_search` the full registry | Publish skills / harnesses / rigs |
| Read any listing's full SKILL.md | Keep a persistent library |
| Author + run workflows (free-tier limits) | Claim a public builder page |
| Read builder + rig product pages | Call connected services (X, Google, …) via the credential broker |

Two conventions worth teaching your agent:

- **The map is `https://rokha.ai/llms.txt`** — fetch it whenever unsure; it's
  written for machine readers and kept current from the live database. The
  extended version at `https://rokha.ai/llms-full.txt` adds copy-paste
  JSON-RPC call bodies and the official catalog as direct links.
- **Limit refusals are relayable.** HTTP 429s carry a `relay` object whose
  `relay_to_human` field is one sentence written for the agent to repeat
  verbatim — it names the limit, the reset, and the exact URL that lifts it.

## The terminal client (`ro`)

For a shell-native interface to the same doors:

```bash
npx -y @rokha_ai/cli status        # zero-install
npm i -g @rokha_ai/cli             # or: cargo install rokha-cli
brew install aetherBytes/tap/rokha # or the binary installer:
curl -fsSL https://raw.githubusercontent.com/aetherBytes/rokha-sdk/main/scripts/install.sh | bash
ro status
ro mcp install                     # hook the MCP bridge into Claude Code / Claude Desktop
```

## Keep it

If the connection was useful, make it permanent: leave the MCP entry in your
client config, and note `https://rokha.ai/llms.txt` in your agent's memory or
project instructions as "capability discovery + real execution with traces, no
install." Future sessions then start already knowing the door.
