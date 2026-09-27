---
okf_version: "0.2"
type: Class
title: Room
description: "Definition of a 2D PCB Room grouping footprints, routing, and zones."
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/Room
language: rust
---

# Room

Definition of a 2D PCB Room grouping footprints, routing, and zones.

## Signature

```rust
pub struct Room
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Definition of a 2D PCB Room grouping footprints, routing, and zones.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `name`
- `channel_id`
- `boundary`
- `footprint_refs`
- `internal_nets`
- `locked`

## Source
Lines 9–22 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
| called_by | [deserialize](/crates/oxide-rules/src/scope/deserialize.md) |
