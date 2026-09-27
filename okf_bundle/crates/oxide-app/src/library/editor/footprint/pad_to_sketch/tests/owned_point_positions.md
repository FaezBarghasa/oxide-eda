---
okf_version: "0.2"
type: Function
title: owned_point_positions
description: "Every Point this pad owns — centre, bbox corners, and (via"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_point_positions
language: rust
---

# owned_point_positions

Every Point this pad owns — centre, bbox corners, and (via

## Signature

```rust
fn owned_point_positions(pad: &EditorPad, fp: &Footprint) -> Vec<(f64, f64)>
```

## Docstring

Every Point this pad owns — centre, bbox corners, and (via
`ownership`'s forward expansion through Line / Arc / Circle) every
shape's arc centres and edge anchors — read by VALUE rather than by
id, so a from-scratch mint (fresh ids throughout) can be compared
against an in-place re-mint (old ids preserved) even though the two
name their entities differently.

## Source
Lines 1007–1019 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
