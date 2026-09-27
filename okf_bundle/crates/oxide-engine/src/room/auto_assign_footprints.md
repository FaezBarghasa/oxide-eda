---
okf_version: "0.2"
type: Function
title: auto_assign_footprints
description: Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/auto_assign_footprints
language: rust
---

# auto_assign_footprints

Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.

## Signature

```rust
impl RoomManager { pub fn auto_assign_footprints(&mut self, board: &PcbBoard) }
```

## Visibility

- `pub`

## Docstring

Auto-assigns footprints on the board to rooms if their origin falls within room boundaries.

## Source
Lines 117–126 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
