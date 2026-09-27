---
okf_version: "0.2"
type: Function
title: points_touched
description: Points referenced (directly or via their parent entity) by a
resource: crates/oxide-sketch/src/solver/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/src/solver/dof/points_touched
language: rust
---

# points_touched

Points referenced (directly or via their parent entity) by a

## Signature

```rust
fn points_touched(kind: &ConstraintKind, sketch: &SketchData) -> Vec<SketchEntityId>
```

## Docstring

Points referenced (directly or via their parent entity) by a
constraint. Used to attribute over-constrained constraints to
per-entity colours.

## Source
Lines 219–293 in `crates/oxide-sketch/src/solver/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/src/solver/dof.md) |
| calls | [extend_with_entity_points](/crates/oxide-sketch/src/solver/dof/extend_with_entity_points.md) |
| called_by | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
