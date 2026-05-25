# Stage 2 probe — stdio MCP servers

Recipe for probing an npm-distributed MCP server in a sandboxed Node env
(Rokha WebContainer, or any local sandbox). Run only after Stage 1 returned
LOW or MEDIUM risk with no FAILs.

## Why a sandbox

`npm install <untrusted-pkg>` runs the package's `postinstall` hooks on
your machine. Don't do that on the user's real OS. Run it where:

- A `rm -rf /` can only delete the sandbox's virtual fs
- A leaked credential exfiltration request goes nowhere outbound that matters
- The whole environment is reset on the next session

In NullBlock today that means a Rokha WebContainer iframe. Outside
NullBlock, that means a fresh Docker container, a microVM, or an isolated
WSL session — the skill works in any of them.

## Recipe

```bash
# Inside the sandbox:
pkg=$1                      # e.g. @smithery/cli-server-fetch
timeout=8                   # seconds before we give up

npm install --no-save "$pkg" 2>&1 | tail -20
node -e "
  const { spawn } = require('child_process');
  const proc = spawn('npx', ['-y', '$pkg'], { stdio: ['pipe', 'pipe', 'inherit'] });
  let buf = '';
  proc.stdout.on('data', d => {
    buf += d.toString();
    // Look for the JSON-RPC response to our tools/list
    const lines = buf.split('\n');
    for (const line of lines) {
      try {
        const msg = JSON.parse(line);
        if (msg.id === 1 && msg.result) {
          console.log(JSON.stringify({ ok: true, tools: msg.result.tools }));
          proc.kill();
          process.exit(0);
        }
      } catch (_) {}
    }
  });
  // initialize + list
  proc.stdin.write(JSON.stringify({ jsonrpc: '2.0', id: 0, method: 'initialize', params: { protocolVersion: '2025-11-25', capabilities: {}, clientInfo: { name: 'rokha-audit', version: '0.1.0' } } }) + '\n');
  proc.stdin.write(JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'tools/list', params: {} }) + '\n');
  setTimeout(() => {
    console.log(JSON.stringify({ ok: false, error: 'probe_timeout' }));
    proc.kill();
    process.exit(1);
  }, $timeout * 1000);
"
```

## What to capture

- The list of tools the server advertises (names + input schemas)
- Any unexpected network calls during install (snapshot the sandbox's
  network log if the runtime supports it; otherwise note "unaudited")
- The package's `postinstall` script content from `package.json` (often
  the most useful single signal for "is this trying to do something
  sketchy")

## Failure modes to handle gracefully

| Symptom | Likely cause | What to say |
|---------|--------------|-------------|
| `npm install` errors out | Native deps the sandbox can't build | "Server requires native deps; can't probe in this sandbox. Audit metadata only." |
| Server starts but never responds | Stdio protocol mismatch | "Server doesn't speak MCP 2025-11-25 — may be old or use a different protocol." |
| `npm install` takes > 30s | Heavy package | Bail and skip probe; note the size in the output. |
| Postinstall script runs network calls | Suspicious | Capture the calls, flag in output regardless of probe outcome. |
