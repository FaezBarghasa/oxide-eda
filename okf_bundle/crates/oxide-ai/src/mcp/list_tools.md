---
okf_version: "0.2"
type: Function
title: list_tools
description: List all supported MCP tools exposed by Oxide EDA.
resource: crates/oxide-ai/src/mcp.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:18:08Z"
concept_id: crates/oxide-ai/src/mcp/list_tools
language: rust
---

# list_tools

List all supported MCP tools exposed by Oxide EDA.

## Signature

```rust
impl McpServer { pub fn list_tools() -> Vec<McpToolInfo> }
```

## Visibility

- `pub`

## Docstring

List all supported MCP tools exposed by Oxide EDA.

## Source
Lines 48–130 in `crates/oxide-ai/src/mcp.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp](/crates/oxide-ai/src/mcp.md) |
