---
okf_version: "0.2"
type: Function
title: line_endpoints
description: "Resolve the start/end Point IDs for a [`EntityKind::Line`]."
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/line_endpoints
language: rust
---

# line_endpoints

Resolve the start/end Point IDs for a [`EntityKind::Line`].

## Signature

```rust
pub fn line_endpoints(
    id: SketchEntityId,
    sketch: &SketchData,
) -> Option<(SketchEntityId, SketchEntityId)>
```

## Visibility

- `pub`

## Docstring

Resolve the start/end Point IDs for a [`EntityKind::Line`].

## Source
Lines 95–104 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
