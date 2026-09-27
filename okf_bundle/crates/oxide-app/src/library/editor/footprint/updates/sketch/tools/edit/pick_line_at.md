---
okf_version: "0.2"
type: Function
title: pick_line_at
description: "Hit-test the click against every sketch Line (0.30 mm tolerance,"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at
language: rust
---

# pick_line_at

Hit-test the click against every sketch Line (0.30 mm tolerance,

## Signature

```rust
fn pick_line_at(sketch: &oxide_sketch::SketchData, x: f64, y: f64) -> Option<SketchEntityId>
```

## Docstring

Hit-test the click against every sketch Line (0.30 mm tolerance,
nearest stroke wins). Shared by both curve-edit tools that pick a
Line by click.

## Source
Lines 42–77 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [fillet_first_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_first_click.md) |
| called_by | [fillet_second_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click.md) |
