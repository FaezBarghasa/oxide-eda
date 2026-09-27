---
okf_version: "0.2"
type: Class
title: RoutingSession
description: Active state while user is dragging or clicking to lay down a trace.
resource: crates/oxide-router/src/interactive/session.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:42:20Z"
concept_id: crates/oxide-router/src/interactive/session/RoutingSession
language: rust
---

# RoutingSession

Active state while user is dragging or clicking to lay down a trace.

## Signature

```rust
pub struct RoutingSession
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Active state while user is dragging or clicking to lay down a trace.
[derive(Debug, Clone)]

## Methods

- `current_net`
- `current_position`
- `current_layer`
- `segments_placed`
- `vias_placed`
- `mode`
- `track_width`
- `tuning_hud`
- `corner_style`

## Source
Lines 84–94 in `crates/oxide-router/src/interactive/session.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session](/crates/oxide-router/src/interactive/session.md) |
