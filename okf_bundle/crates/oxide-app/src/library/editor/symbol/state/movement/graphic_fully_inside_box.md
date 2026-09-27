---
okf_version: "0.2"
type: Function
title: graphic_fully_inside_box
resource: crates/oxide-app/src/library/editor/symbol/state/movement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/movement/graphic_fully_inside_box
language: rust
---

# graphic_fully_inside_box

## Signature

```rust
fn graphic_fully_inside_box(
    kind: &SymbolGraphicKind,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
) -> bool
```

## Source
Lines 275–307 in `crates/oxide-app/src/library/editor/symbol/state/movement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [movement](/crates/oxide-app/src/library/editor/symbol/state/movement.md) |
| calls | [polygon_bbox](/crates/oxide-app/src/library/editor/symbol/state/movement/polygon_bbox.md) |
| called_by | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
