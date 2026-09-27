---
okf_version: "0.2"
type: Function
title: full_turn_pair_is_detected
description: "A `0 -> 360°` (`0 -> TAU`) pair sweeps zero CCW but spans a full"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/full_turn_pair_is_detected
language: rust
---

# full_turn_pair_is_detected

A `0 -> 360°` (`0 -> TAU`) pair sweeps zero CCW but spans a full

## Signature

```rust
fn full_turn_pair_is_detected()
```

## Decorators

- `test`

## Docstring

A `0 -> 360°` (`0 -> TAU`) pair sweeps zero CCW but spans a full
turn: it is a circle, not a point. Both the draw path and the
hit-test must agree on this via `arc_is_full_turn_rad`.
[test]

## Source
Lines 121–129 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
