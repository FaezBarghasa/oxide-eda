---
okf_version: "0.2"
type: Function
title: arc_refs
description: Resolve the centre/start/end Point IDs and CCW flag for an
resource: crates/oxide-sketch/src/solver/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/state/arc_refs
language: rust
---

# arc_refs

Resolve the centre/start/end Point IDs and CCW flag for an

## Signature

```rust
pub fn arc_refs(
    id: SketchEntityId,
    sketch: &SketchData,
) -> Option<(SketchEntityId, SketchEntityId, SketchEntityId, bool)>
```

## Visibility

- `pub`

## Docstring

Resolve the centre/start/end Point IDs and CCW flag for an
[`EntityKind::Arc`].

## Source
Lines 108–122 in `crates/oxide-sketch/src/solver/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-sketch/src/solver/state.md) |
| called_by | [entity_radius](/crates/oxide-sketch/src/solver/residuals/equal_tangent/entity_radius.md) |
| called_by | [point_on_arc](/crates/oxide-sketch/src/solver/residuals/point_on/point_on_arc.md) |
