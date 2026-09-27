---
okf_version: "0.2"
type: Class
title: SymbolToolMsg
description: Tool selection on the Symbol canvas — pure-data alias for the
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/SymbolToolMsg
language: rust
---

# SymbolToolMsg

Tool selection on the Symbol canvas — pure-data alias for the

## Signature

```rust
pub enum SymbolToolMsg
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Tool selection on the Symbol canvas — pure-data alias for the
canvas's own `SymbolTool` so messages don't depend on the canvas
module type tree.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 294–303 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
