from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Literal
import httpx

DEFAULT_BASE_URL = "http://localhost:3000"
DEFAULT_TIMEOUT = 30.0

SchemaCompatLevel = Literal["match", "minor-drift", "major-drift", "unreachable"]


@dataclass
class SchemaCompatReport:
    level: SchemaCompatLevel
    client: str
    server: str | None
    message: str


class RokhaError(Exception):
    def __init__(self, status: int, body: str, path: str):
        self.status = status
        self.body = body
        self.path = path
        super().__init__(f"Rokha API error {status} on {path}: {body}")


class RokhaClient:
    SCHEMA_VERSION = "4.6.0"

    def __init__(
        self,
        base_url: str = DEFAULT_BASE_URL,
        wallet_address: str | None = None,
        wallet_chain: str | None = None,
        timeout: float = DEFAULT_TIMEOUT,
        auth_token: str | None = None,
    ):
        self.base_url = base_url.rstrip("/")
        self.wallet_address = wallet_address
        self.wallet_chain = wallet_chain
        self.auth_token = auth_token
        self._http = httpx.Client(base_url=self.base_url, timeout=timeout)

    def set_auth_token(self, jwt: str | None) -> None:
        self.auth_token = jwt

    def _headers(self) -> dict[str, str]:
        h: dict[str, str] = {}
        if self.wallet_address:
            h["x-wallet-address"] = self.wallet_address
        if self.wallet_chain:
            h["x-wallet-chain"] = self.wallet_chain
        if self.auth_token:
            h["Authorization"] = f"Bearer {self.auth_token}"
        return h

    def _request(self, method: str, path: str, **kwargs: Any) -> Any:
        headers = {**self._headers(), **kwargs.pop("headers", {})}
        res = self._http.request(method, path, headers=headers, **kwargs)
        if res.status_code >= 400:
            raise RokhaError(res.status_code, res.text, path)
        if res.headers.get("content-type", "").startswith("application/json"):
            return res.json()
        return res.text

    def get(self, path: str, **params: Any) -> Any:
        return self._request("GET", path, params=params or None)

    def post(self, path: str, body: Any = None) -> Any:
        return self._request("POST", path, json=body)

    def put(self, path: str, body: Any = None) -> Any:
        return self._request("PUT", path, json=body)

    def delete(self, path: str) -> None:
        self._request("DELETE", path)

    def health(self) -> dict[str, Any]:
        return self.get("/health")

    def get_skill_md(self, provider: str, slug: str) -> dict[str, Any]:
        """Ingest a registry listing's real SKILL.md (schema 4.6.0).

        Fetched from its source registry, parsed, and classified
        server-side. ``classification`` answers "can a model execute this
        faithfully?" — ``prompt`` yes; ``scripted``/``mcp`` need a runtime.
        No auth.
        """
        return self.get("/api/marketplace/registry/skill-md", provider=provider, slug=slug)

    def check_schema_compat(self) -> SchemaCompatReport:
        """Verify the live Erebus schema version matches this SDK's expected version.

        Returns a report; the SDK does not raise on drift — caller decides.
        """
        client = self.SCHEMA_VERSION
        try:
            res = self.get("/api/schema/version")
            server = res["version"]
        except Exception:
            return SchemaCompatReport(
                level="unreachable",
                client=client,
                server=None,
                message=f"Could not fetch {self.base_url}/api/schema/version",
            )
        if server == client:
            return SchemaCompatReport(level="match", client=client, server=server, message="OK")
        if server.split(".")[0] != client.split(".")[0]:
            return SchemaCompatReport(
                level="major-drift",
                client=client,
                server=server,
                message=f"Incompatible: server {server}, SDK {client}. Upgrade rokha-sdk.",
            )
        return SchemaCompatReport(
            level="minor-drift",
            client=client,
            server=server,
            message=f"Server {server}, SDK {client}. Forward-compatible; some endpoints may be new.",
        )

    # --- CLI device flow (ro login) ---

    def cli_auth_start(self, scope: str = "cli", client: str | None = None) -> dict[str, Any]:
        body: dict[str, Any] = {"scope": scope}
        if client:
            body["client"] = client
        return self.post("/api/auth/cli/start", body)

    def cli_auth_poll(self, device_code: str) -> dict[str, Any]:
        return self.post("/api/auth/cli/poll", {"device_code": device_code})

    def cli_auth_authorize(self, user_code: str) -> dict[str, Any]:
        """Requires `set_auth_token(jwt)` for an existing user session."""
        return self.post("/api/auth/cli/authorize", {"user_code": user_code})

    # --- LLM proxy (federated; uses caller's BYO key or Rokha tenant key) ---

    def llm_proxy(self, body: dict[str, Any]) -> dict[str, Any]:
        """Forward an Anthropic /v1/messages request through Rokha.

        Body and response mirror Anthropic's shape verbatim. The caller's
        Bearer JWT is sent automatically (set via `set_auth_token`).
        BYO Anthropic keys win; otherwise the Rokha tenant key is used
        with the free-tier daily rate limit applied.
        """
        return self.post("/api/v1/llm/proxy", body)

    # --- Agents ---

    def agent_chat(self, agent: str, message: str, **kwargs: Any) -> dict[str, Any]:
        return self.post(f"/api/agents/{agent}/chat", {"message": message, **kwargs})

    def agent_status(self, agent: str = "rokha-agent") -> dict[str, Any]:
        return self.get(f"/api/agents/{agent}/status")

    def agent_tools(self, agent: str = "rokha-agent") -> list[dict[str, Any]]:
        return self.get(f"/api/agents/{agent}/tools")

    # --- Harnesses ---

    def list_harnesses(self, **params: Any) -> list[dict[str, Any]]:
        return self.get("/api/harnesses", **params)

    def get_harness(self, harness_id: str) -> dict[str, Any]:
        return self.get(f"/api/harnesses/{harness_id}")

    def create_harness(self, wallet_address: str, harness_type: str, content: dict[str, Any], tags: list[str] | None = None) -> dict[str, Any]:
        body: dict[str, Any] = {"wallet_address": wallet_address, "harness_type": harness_type, "content": content}
        if tags:
            body["tags"] = tags
        return self.post("/api/harnesses", body)

    def update_harness(self, harness_id: str, **updates: Any) -> dict[str, Any]:
        return self.put(f"/api/harnesses/{harness_id}", updates)

    def delete_harness(self, harness_id: str) -> None:
        self.delete(f"/api/harnesses/{harness_id}")

    # --- MCP ---

    def mcp_list_tools(self) -> list[dict[str, Any]]:
        return self.get("/mcp/tools")

    def mcp_call_tool(self, name: str, arguments: dict[str, Any] | None = None) -> Any:
        return self.post("/mcp/jsonrpc", {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {"name": name, "arguments": arguments or {}},
        })

    # --- Marketplace ---

    def list_listings(self) -> list[dict[str, Any]]:
        return self.get("/api/marketplace/listings")

    def search_listings(self, query: str | None = None, listing_type: str | None = None) -> list[dict[str, Any]]:
        body: dict[str, Any] = {}
        if query:
            body["query"] = query
        if listing_type:
            body["listing_type"] = listing_type
        return self.post("/api/marketplace/search", body)

    # --- Discovery ---

    def discover_all(self) -> dict[str, Any]:
        return self.get("/api/discovery/all")

    def discover_recent(self, limit: int = 20) -> dict[str, Any]:
        """Recent mesh activity (cross-registry events, newest first). No auth."""
        return self.get("/api/discovery/recent", limit=limit)

    def mcp_proxy(self, endpoint: str, method: str, params: dict[str, Any] | None = None) -> dict[str, Any]:
        """Stateless one-shot outbound MCP probe/call via the SSRF-guarded proxy.

        `method` is 'tools/list' (public) or 'tools/call' (requires the
        client's Bearer JWT; params = {"name": ..., "arguments": {...}}).
        """
        return self.post("/api/marketplace/mcp-proxy", {"endpoint": endpoint, "method": method, "params": params or {}})

    # --- Rigs + Traces (the Working Rig surface) ---
    # Authed: owner-scoped via the Bearer JWT. Pre-login: pass
    # anon_session_id to hit the /api/anon mirror (x-anon-session-id header).

    def _rig_prefix_headers(self, anon_session_id: str | None) -> tuple[str, dict[str, str]]:
        if anon_session_id:
            return "/api/anon", {"x-anon-session-id": anon_session_id}
        return "/api", {}

    def list_rigs(self, anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("GET", f"{prefix}/rigs", headers=headers)

    def get_rig(self, rig_id: str, anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("GET", f"{prefix}/rigs/{rig_id}", headers=headers)

    def create_rig(self, body: dict[str, Any], anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("POST", f"{prefix}/rigs", json=body, headers=headers)

    def update_rig(self, rig_id: str, body: dict[str, Any], anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("PUT", f"{prefix}/rigs/{rig_id}", json=body, headers=headers)

    def list_traces(self, limit: int = 50, offset: int = 0, anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("GET", f"{prefix}/traces", params={"limit": limit, "offset": offset}, headers=headers)

    def get_trace(self, trace_id: str, anon_session_id: str | None = None) -> dict[str, Any]:
        prefix, headers = self._rig_prefix_headers(anon_session_id)
        return self._request("GET", f"{prefix}/traces/{trace_id}", headers=headers)

    def close(self) -> None:
        self._http.close()

    def __enter__(self) -> RokhaClient:
        return self

    def __exit__(self, *_: Any) -> None:
        self.close()
