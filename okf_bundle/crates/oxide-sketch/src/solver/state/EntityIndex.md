---
okf_version: "0.2"
type: Class
title: EntityIndex
description: "Maps a Point's `SketchEntityId` to its `(x, y)` offset in the"
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/EntityIndex
language: rust
---

# EntityIndex

Maps a Point's `SketchEntityId` to its `(x, y)` offset in the

## Signature

```rust
pub struct EntityIndex
```

## Decorators

- `derive(Debug, Default, Clone)`

## Visibility

- `pub`

## Docstring

Maps a Point's `SketchEntityId` to its `(x, y)` offset in the
state vector, plus per-circle radius offsets and the set of
Fixed-constrained points whose coords are read directly from the
`Entity` instead of the state vector.
[derive(Debug, Default, Clone)]

## Methods

- `points`
- `radii`
- `fixed`

## Source
Lines 13–17 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
