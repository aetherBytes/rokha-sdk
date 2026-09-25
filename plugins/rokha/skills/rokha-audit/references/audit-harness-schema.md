# Audit harness schema

The structured body saved as the harness `content` after a `rokha-audit` run.
Stored as JSON-stringified content in a `strategy`-type harness so it can be
searched, forked, and replayed alongside other workflow memories.

```json
{
  "tool_name": "Math-MCP",
  "source": "smithery",
  "input_config": {
    "endpoint": "https://server.smithery.ai/.../mcp",
    "probe_args": {},
    "options": { "timeout_ms": 5000 }
  },
  "scan_results": {
    "schema":      { "status": "WARN", "reason": "No inputSchema defined" },
    "endpoint":    { "status": "PASS", "reason": "HTTPS endpoint" },
    "permissions": { "status": "PASS", "reason": "No sensitive keywords" },
    "metadata":    { "status": "WARN", "reason": "not verified" },
    "install":     { "status": "PASS", "reason": "registry-hosted" },
    "trust":       { "status": "WARN", "reason": "zero stars" }
  },
  "probe_results": {
    "probed": true,
    "method": "http",                   // "http" | "npm-stdio"
    "package": null,                    // npm package name (npm-stdio only)
    "tools_found": 4,
    "summary": "Endpoint responded with 4 tool(s).",
    "tools": [ { "name": "add", "inputSchema": { ... } }, ... ],
    "install_ms": null,                 // npm-stdio only
    "postinstall_hooks": null,          // null or { preinstall?, install?, postinstall? }
    "error": null                       // present when probe fails: install_timeout | install_failed | probe_failed | unexpected_response | connection_error
  },
  "risk_level": "MEDIUM",
  "timestamp": 1778680000000,
  "run_count": 1
}
```

## Field notes

- `risk_level` — derived: `HIGH` if ≥2 FAILs, `MEDIUM` if 1 FAIL or ≥3 WARNs, `LOW` otherwise
- `probe_results.probed` — false if Stage 2 was skipped (no endpoint and no derivable npm package); the rest of the
  block still renders so re-audits can fill it in later
- `probe_results.method` — `"http"` for HTTPS-endpoint MCPs (Erebus-proxied),
  `"npm-stdio"` for npm-installable MCPs probed inside the Rokha sandbox,
  `null` if probe skipped
- `probe_results.postinstall_hooks` — captured from the installed
  package.json's `scripts` field (preinstall / install / postinstall).
  Presence does NOT automatically fail TRUST but warrants human review
- `run_count` — incremented on re-audit; never reset

## Why an harness

- Wallet-scoped (private to the user/agent that ran the audit)
- Searchable by tag (`["tool","audit","nb-audit"]`) so a future flow can
  recall "what have I already audited?"
- Forkable — a user can hand their audit results to a teammate without
  re-running the probe
