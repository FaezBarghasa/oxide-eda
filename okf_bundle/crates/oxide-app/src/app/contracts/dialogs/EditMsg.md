---
okf_version: "0.2"
type: Class
title: EditMsg
description: Edit-command message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/EditMsg
language: rust
---

# EditMsg

Edit-command message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum EditMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Edit-command message family (ADR-0001 D3). Namespaced under
`Message::Edit` and routed to `dispatch_edit_message`.
[derive(Debug, Clone)]

## Source
Lines 335–360 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
