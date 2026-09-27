---
okf_version: "0.2"
type: Class
title: MoveSelectionMsg
description: Move Selection dialog message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/MoveSelectionMsg
language: rust
---

# MoveSelectionMsg

Move Selection dialog message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum MoveSelectionMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Move Selection dialog message family (ADR-0001 D3). Namespaced under
`Message::MoveSelection` and routed to `dispatch_move_selection_message`.
[derive(Debug, Clone)]

## Source
Lines 254–266 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
