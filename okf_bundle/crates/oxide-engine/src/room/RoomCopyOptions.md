---
okf_version: "0.2"
type: Class
title: RoomCopyOptions
description: Options controlling which elements are formatted/replicated when copying room formats.
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/RoomCopyOptions
language: rust
---

# RoomCopyOptions

Options controlling which elements are formatted/replicated when copying room formats.

## Signature

```rust
pub struct RoomCopyOptions
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Options controlling which elements are formatted/replicated when copying room formats.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `copy_footprint_placement`
- `copy_routing_traces`
- `copy_vias`
- `copy_zones`

## Source
Lines 75–80 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
