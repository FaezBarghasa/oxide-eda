---
okf_version: "0.2"
type: Function
title: translate_graphic_to
description: "Translate the graphic at `idx` so its primary anchor lands on"
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/translate_graphic_to
language: rust
---

# translate_graphic_to

Translate the graphic at `idx` so its primary anchor lands on

## Signature

```rust
pub(super) fn translate_graphic_to(sym: &mut Symbol, idx: usize, x: f64, y: f64)
```

## Visibility

- `pub(super)`

## Docstring

Translate the graphic at `idx` so its primary anchor lands on
`(x, y)`. Anchors are picked to match the visual centre of mass
of each shape: rectangles + lines move by the delta between the
new point and `from`; circles + arcs use `center`; text uses
`position`. No-op when `idx` is out of range.

## Source
Lines 269–310 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
| called_by | [move_selected](/crates/oxide-app/src/library/editor/symbol/state/movement/move_selected.md) |
