---
okf_version: "0.2"
type: Class
title: SymbolContextTargetMsg
description: What the cursor was over at right-click time — pure-data alias of
resource: crates/oxide-app/src/library/messages/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/mod/SymbolContextTargetMsg
language: rust
---

# SymbolContextTargetMsg

What the cursor was over at right-click time — pure-data alias of

## Signature

```rust
pub enum SymbolContextTargetMsg
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

What the cursor was over at right-click time — pure-data alias of
`editor::symbol::state::SymbolContextTarget`. Carried by
`SymbolEditorMsg::ShowContextMenu`.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 515–519 in `crates/oxide-app/src/library/messages/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/messages/mod.md) |
