---
okf_version: "0.2"
type: Class
title: EnableVersionControlMsg
description: Enable Version Control modal message family (ADR-0001 D3).
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/EnableVersionControlMsg
language: rust
---

# EnableVersionControlMsg

Enable Version Control modal message family (ADR-0001 D3).

## Signature

```rust
pub enum EnableVersionControlMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Enable Version Control modal message family (ADR-0001 D3).
Namespaced under `Message::EnableVersionControl` and routed to
`dispatch_enable_version_control_message`.
[derive(Debug, Clone)]

## Source
Lines 106–120 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
