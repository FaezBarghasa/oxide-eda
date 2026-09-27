---
okf_version: "0.2"
type: Function
title: center_of
description: "Resolve the centre Point ID for either an [`EntityKind::Arc`] or"
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/center_of
language: rust
---

# center_of

Resolve the centre Point ID for either an [`EntityKind::Arc`] or

## Signature

```rust
pub fn center_of(id: SketchEntityId, sketch: &SketchData) -> Option<SketchEntityId>
```

## Visibility

- `pub`

## Docstring

Resolve the centre Point ID for either an [`EntityKind::Arc`] or
[`EntityKind::Circle`].

## Source
Lines 126–133 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
