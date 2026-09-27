---
okf_version: "0.2"
type: Module
title: mcp
description: Oxide Model Context Protocol (MCP) Server Surface (Phase 5.4).
resource: crates/oxide-ai/src/mcp.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:18:08Z"
concept_id: crates/oxide-ai/src/mcp
language: rust
---

# mcp

Oxide Model Context Protocol (MCP) Server Surface (Phase 5.4).

## Docstring

Oxide Model Context Protocol (MCP) Server Surface (Phase 5.4).

Exposes structured EDA queries and commands over standard Model Context Protocol (MCP)
for automated AI agent interaction, natural-language design synthesis, DRC verification,
and automated routing.

## Relationships

| Type | Target |
|------|--------|
| related | [McpRequest](/crates/oxide-ai/src/mcp/McpRequest.md) |
| related | [McpResponse](/crates/oxide-ai/src/mcp/McpResponse.md) |
| related | [McpError](/crates/oxide-ai/src/mcp/McpError.md) |
| related | [McpToolInfo](/crates/oxide-ai/src/mcp/McpToolInfo.md) |
| related | [McpServer](/crates/oxide-ai/src/mcp/McpServer.md) |
| related | [list_tools](/crates/oxide-ai/src/mcp/list_tools.md) |
| related | [handle_request](/crates/oxide-ai/src/mcp/handle_request.md) |
| related | [list_tools](/crates/oxide-ai/src/mcp/list_tools.md) |
| related | [handle_request](/crates/oxide-ai/src/mcp/handle_request.md) |
| related | [test_list_tools](/crates/oxide-ai/src/mcp/test_list_tools.md) |
| related | [test_handle_mcp_place_component](/crates/oxide-ai/src/mcp/test_handle_mcp_place_component.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
