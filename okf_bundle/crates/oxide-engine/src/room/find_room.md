---
okf_version: "0.2"
type: Function
title: find_room
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/find_room
language: rust
---

# find_room

## Signature

```rust
impl RoomManager { pub fn find_room(&self, id: Uuid) -> Option<&Room> }
```

## Visibility

- `pub`

## Source
Lines 108–110 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
