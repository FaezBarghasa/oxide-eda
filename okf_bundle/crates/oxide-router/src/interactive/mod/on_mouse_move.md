---
okf_version: "0.2"
type: Function
title: on_mouse_move
description: Process mouse pointer movement to a target coordinate.
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/on_mouse_move
language: rust
---

# on_mouse_move

Process mouse pointer movement to a target coordinate.

## Signature

```rust
impl InteractiveRouter { pub fn on_mouse_move(&mut self, target: Point2D) -> Vec<RouteSegment> }
```

## Visibility

- `pub`

## Docstring

Process mouse pointer movement to a target coordinate.

## Source
Lines 156–191 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
