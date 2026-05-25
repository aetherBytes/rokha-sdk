import type { RokhaClient } from './client.js';
import type { MCPToolInfo, MCPJsonRpcRequest, MCPJsonRpcResponse } from './types.js';

let nextId = 1;

export class MCPClient {
  constructor(private client: RokhaClient) {}

  async health(): Promise<{ status: string }> {
    return this.client.get('/mcp/health');
  }

  async listTools(): Promise<MCPToolInfo[]> {
    return this.client.get('/mcp/tools');
  }

  async listResources(): Promise<unknown[]> {
    return this.client.get('/mcp/resources');
  }

  async listPrompts(): Promise<unknown[]> {
    return this.client.get('/mcp/prompts');
  }

  async callTool(name: string, args: Record<string, unknown> = {}): Promise<unknown> {
    const response = await this.jsonrpc('tools/call', { name, arguments: args });
    if (response.error) {
      throw new Error(`MCP tool error ${response.error.code}: ${response.error.message}`);
    }
    return response.result;
  }

  async initialize(): Promise<MCPJsonRpcResponse> {
    return this.jsonrpc('initialize', {
      protocolVersion: '2025-11-25',
      capabilities: {},
      clientInfo: { name: '@rokha/sdk', version: '0.1.0' },
    });
  }

  async jsonrpc(method: string, params: Record<string, unknown> = {}): Promise<MCPJsonRpcResponse> {
    const request: MCPJsonRpcRequest = {
      jsonrpc: '2.0',
      id: nextId++,
      method,
      params,
    };
    return this.client.post('/mcp/jsonrpc', request);
  }
}
