---
okf_version: "0.2"
type: Function
title: add_entity
description: "Build an [`AddEntity`](SketchEdit::AddEntity) edit from a plain"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/messages/add_entity_1
language: rust
---

# add_entity

Build an [`AddEntity`](SketchEdit::AddEntity) edit from a plain

## Signature

```rust
pub fn add_entity(entity: Entity) -> Self
```

## Visibility

- `pub`

## Docstring

Build an [`AddEntity`](SketchEdit::AddEntity) edit from a plain
`Entity`. Callers mint entities on the stack; this is the single
place that moves one into the variant's box.

## Source
Lines 69–71 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [messages](/crates/oxide-app/src/library/editor/footprint/sketch_mode/messages.md) |
