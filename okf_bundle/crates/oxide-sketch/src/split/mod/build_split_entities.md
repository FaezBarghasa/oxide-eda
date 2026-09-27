---
okf_version: "0.2"
type: Function
title: build_split_entities
description: "Mint the mid `Point` and the two replacement `Line`s, each"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/build_split_entities
language: rust
---

# build_split_entities

Mint the mid `Point` and the two replacement `Line`s, each

## Signature

```rust
fn build_split_entities(
    template: Entity,
    start_id: SketchEntityId,
    end_id: SketchEntityId,
    mid_id: SketchEntityId,
    mid_xy: (f64, f64),
    line_a_id: SketchEntityId,
    line_b_id: SketchEntityId,
) -> (Entity, Entity, Entity)
```

## Docstring

Mint the mid `Point` and the two replacement `Line`s, each
inheriting `template`'s plane and every bake attribute (`template`
is the retired Line entity itself, cloned before removal).

## Source
Lines 230–272 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| called_by | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
