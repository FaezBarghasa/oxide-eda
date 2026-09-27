---
okf_version: "0.2"
type: Function
title: graphic_handle_position
description: "Compute the world (mm) position of a graphic's resize handle."
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/graphic_handle_position
language: rust
---

# graphic_handle_position

Compute the world (mm) position of a graphic's resize handle.

## Signature

```rust
pub fn graphic_handle_position(
    sym: &Symbol,
    idx: usize,
    handle: GraphicHandle,
) -> Option<[f64; 2]>
```

## Visibility

- `pub`

## Docstring

Compute the world (mm) position of a graphic's resize handle.
Returns `None` if the handle variant doesn't match the graphic
kind — defensive against stale `GraphicHandle` values lingering
across selection swaps.

## Source
Lines 201–260 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
