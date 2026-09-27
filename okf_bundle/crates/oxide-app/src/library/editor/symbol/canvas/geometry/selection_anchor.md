---
okf_version: "0.2"
type: Function
title: selection_anchor
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry/selection_anchor
language: rust
---

# selection_anchor

## Signature

```rust
pub(super) fn selection_anchor(symbol: &Symbol, selection: &SymbolSelection) -> Option<(f64, f64)>
```

## Visibility

- `pub(super)`

## Source
Lines 108–132 in `crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/symbol/canvas/geometry.md) |
| calls | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
