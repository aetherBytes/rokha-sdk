use crate::api_client::RokhaClient;
use std::io::{self, BufRead, Write};

pub async fn serve(_client: &RokhaClient) {
    eprintln!("Rokha MCP server running on stdio");
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        let req: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let id = req.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = req["method"].as_str().unwrap_or("");

        let response = match method {
            "initialize" => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "rokha-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            }),
            "tools/list" => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "rokha_health",
                            "description": "Check Rokha service health",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "rokha_tools_list",
                            "description": "List available tools in the Rokha Registry",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        },
                        {
                            "name": "rokha_chat",
                            "description": "Send a message to the Rokha agent",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "message": {
                                        "type": "string",
                                        "description": "Message to send"
                                    }
                                },
                                "required": ["message"]
                            }
                        }
                    ]
                }
            }),
            _ => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            }),
        };

        let out = serde_json::to_string(&response).unwrap();
        let _ = writeln!(stdout, "{}", out);
        let _ = stdout.flush();
    }
}
