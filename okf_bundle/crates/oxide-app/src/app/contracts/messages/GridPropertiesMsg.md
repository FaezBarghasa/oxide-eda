---
okf_version: "0.2"
type: Class
title: GridPropertiesMsg
description: Grid Properties dialog message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/GridPropertiesMsg
language: rust
---

# GridPropertiesMsg

Grid Properties dialog message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum GridPropertiesMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Grid Properties dialog message family (ADR-0001 D3). Namespaced
under `Message::GridProperties` and routed to
`dispatch_grid_properties_message`.
[derive(Debug, Clone)]

## Source
Lines 400–428 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
