---
okf_version: "0.2"
type: Class
title: FootprintCanvas
description: "The Canvas program. Holds a snapshot of the model — `view()`"
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/FootprintCanvas
language: rust
---

# FootprintCanvas

The Canvas program. Holds a snapshot of the model — `view()`

## Signature

```rust
pub struct FootprintCanvas
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

The Canvas program. Holds a snapshot of the model — `view()`
rebuilds this every frame, so we only need a borrowed reference.

## Methods

- `state`
- `address`
- `bg_color`
- `grid_color`
- `cache`
- `sketch`
- `silk_f`
- `silk_b`

## Source
Lines 204–224 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
