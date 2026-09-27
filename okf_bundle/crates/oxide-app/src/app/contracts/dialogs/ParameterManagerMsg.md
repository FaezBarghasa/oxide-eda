---
okf_version: "0.2"
type: Class
title: ParameterManagerMsg
description: Parameter Manager dialog message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/ParameterManagerMsg
language: rust
---

# ParameterManagerMsg

Parameter Manager dialog message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum ParameterManagerMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Parameter Manager dialog message family (ADR-0001 D3). Namespaced
under `Message::ParameterManager` and routed to
`dispatch_parameter_manager_message`.
[derive(Debug, Clone)]

## Methods

- `symbol_uuid`
- `key`
- `value`

## Source
Lines 34–45 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
