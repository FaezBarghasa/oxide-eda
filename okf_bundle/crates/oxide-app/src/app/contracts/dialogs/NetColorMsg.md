---
okf_version: "0.2"
type: Class
title: NetColorMsg
description: Per-net colour override message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/dialogs.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/dialogs/NetColorMsg
language: rust
---

# NetColorMsg

Per-net colour override message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum NetColorMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Per-net colour override message family (ADR-0001 D3). Namespaced
under `Message::NetColor` and routed to
`dispatch_net_color_message`.
[derive(Debug, Clone)]

## Methods

- `net`
- `color`

## Source
Lines 9–28 in `crates/oxide-app/src/app/contracts/dialogs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dialogs](/crates/oxide-app/src/app/contracts/dialogs.md) |
