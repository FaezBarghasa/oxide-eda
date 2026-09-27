---
okf_version: "0.2"
type: Class
title: RenameMsg
description: Rename modal message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/RenameMsg
language: rust
---

# RenameMsg

Rename modal message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum RenameMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Rename modal message family (ADR-0001 D3). Namespaced under
`Message::Rename` and routed to `dispatch_rename_message`.
[derive(Debug, Clone)]

## Source
Lines 125–133 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
