---
okf_version: "0.2"
type: Function
title: new
description: Construct a bare entity with no bake attributes attached.
resource: crates/oxide-sketch/src/entity.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/entity/new_1
language: rust
---

# new

Construct a bare entity with no bake attributes attached.

## Signature

```rust
pub fn new(id: SketchEntityId, plane: PlaneId, kind: EntityKind) -> Self
```

## Visibility

- `pub`

## Docstring

Construct a bare entity with no bake attributes attached.

## Source
Lines 90–108 in `crates/oxide-sketch/src/entity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entity](/crates/oxide-sketch/src/entity.md) |
