---
okf_version: "0.2"
type: Function
title: handle_request
description: Dispatch and execute a tool call against the active project context.
resource: crates/oxide-ai/src/mcp.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ai"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:18:08Z"
concept_id: crates/oxide-ai/src/mcp/handle_request
language: rust
---

# handle_request

Dispatch and execute a tool call against the active project context.

## Signature

```rust
impl McpServer { pub fn handle_request(
        req: &McpRequest,
        _sheet: Option<&SchematicSheet>,
        board: Option<&PcbBoard>,
    ) -> McpResponse }
```

## Visibility

- `pub`

## Docstring

Dispatch and execute a tool call against the active project context.

## Source
Lines 133–210 in `crates/oxide-ai/src/mcp.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcp](/crates/oxide-ai/src/mcp.md) |
