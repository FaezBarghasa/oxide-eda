---
okf_version: "0.2"
type: Class
title: OverlayMsg
description: Overlay / modal-chrome message family (ADR-0001 D3). Namespaced
resource: crates/oxide-app/src/app/contracts/messages.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/contracts/messages/OverlayMsg
language: rust
---

# OverlayMsg

Overlay / modal-chrome message family (ADR-0001 D3). Namespaced

## Signature

```rust
pub enum OverlayMsg
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Overlay / modal-chrome message family (ADR-0001 D3). Namespaced
under `Message::Overlay` and routed to `dispatch_overlay_message`.
[derive(Debug, Clone)]

## Methods

- `modal`
- `x`
- `y`
- `world_x`
- `world_y`
- `select`

## Source
Lines 142–191 in `crates/oxide-app/src/app/contracts/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/app/contracts/messages.md) |
