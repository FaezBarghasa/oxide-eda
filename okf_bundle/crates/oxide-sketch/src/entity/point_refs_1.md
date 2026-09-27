---
okf_version: "0.2"
type: Function
title: point_refs
description: Point endpoints reachable from this entity. Used by the
resource: crates/oxide-sketch/src/entity.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/entity/point_refs_1
language: rust
---

# point_refs

Point endpoints reachable from this entity. Used by the

## Signature

```rust
pub fn point_refs(&self) -> Vec<SketchEntityId>
```

## Visibility

- `pub`

## Docstring

Point endpoints reachable from this entity. Used by the
solver to discover entity → state-vector mapping.

## Source
Lines 120–129 in `crates/oxide-sketch/src/entity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entity](/crates/oxide-sketch/src/entity.md) |
