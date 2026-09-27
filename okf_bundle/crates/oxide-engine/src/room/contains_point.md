---
okf_version: "0.2"
type: Function
title: contains_point
description: "Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm)."
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/contains_point
language: rust
---

# contains_point

Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm).

## Signature

```rust
impl Room { pub fn contains_point(&self, pt: Point) -> bool }
```

## Visibility

- `pub`

## Docstring

Checks if a 2D point lies within the room's boundary polygon (ray-casting algorithm).

## Source
Lines 38–56 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
