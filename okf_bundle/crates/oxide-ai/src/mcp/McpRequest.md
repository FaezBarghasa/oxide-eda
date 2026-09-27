---
okf_version: "0.2"
type: Class
title: McpRequest
description: Standard MCP Request Envelope.
resource: crates/oxide-ai/src/mcp.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:18:08Z"
concept_id: crates/oxide-ai/src/mcp/McpRequest
language: rust
---

# McpRequest

Standard MCP Request Envelope.

## Signature

```rust
pub struct McpRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Standard MCP Request Envelope.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `method`
- `params`

## Source
Lines 14–19 in `crates/oxide-ai/src/mcp.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp](/crates/oxide-ai/src/mcp.md) |
