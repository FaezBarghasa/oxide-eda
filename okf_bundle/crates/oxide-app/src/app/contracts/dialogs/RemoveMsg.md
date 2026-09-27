---
okf_version: "0.2"
type: Class
title: RemoveMsg
description: Remove-from-project modal message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/RemoveMsg
language: rust
---

# RemoveMsg

Remove-from-project modal message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum RemoveMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Remove-from-project modal message family (ADR-0001 D3). Namespaced
under `Message::Remove` and routed to `dispatch_remove_message`.
[derive(Debug, Clone)]

## Source
Lines 138–143 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
