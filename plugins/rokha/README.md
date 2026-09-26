# Rokha — Agent Plugin

One plugin, every client that speaks the [Agent Plugins](https://agent-plugins.org) standard
(GitHub Copilot CLI, VS Code, the Copilot app, Claude Code).

It gives your agent two things:

- **The Rokha MCP server** (`https://rokha.ai/mcp/jsonrpc`, streamable HTTP). Search and
  fetch work anonymously; anything that runs or writes asks you to sign in once through
  OAuth (the server advertises its authorization server, clients register themselves).
- **The first-party skills** in `skills/` — the same folders as the repo's top-level
  `skills/`, copied in because a plugin's files must live inside the plugin root.

## Install

```bash
copilot plugin marketplace add rokha-ai/rokha-sdk
copilot plugin install rokha
```

Claude Code: `/plugin marketplace add rokha-ai/rokha-sdk` then `/plugin install rokha`.

## Keep the copy honest

`skills/` here is a mirror. After editing a top-level skill, run
`./scripts/sync-plugin-skills.sh` and commit both.
