---
okf_version: "0.2"
type: Class
title: ErcMsg
description: ERC dialog message family (ADR-0001 D3). Namespaced under
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/ErcMsg
language: rust
---

# ErcMsg

ERC dialog message family (ADR-0001 D3). Namespaced under

## Signature

```rust
pub enum ErcMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

ERC dialog message family (ADR-0001 D3). Namespaced under
`Message::Erc` and routed to `dispatch_erc_message`.
[derive(Debug, Clone)]

## Methods

- `row`
- `col`

## Source
Lines 73–86 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
