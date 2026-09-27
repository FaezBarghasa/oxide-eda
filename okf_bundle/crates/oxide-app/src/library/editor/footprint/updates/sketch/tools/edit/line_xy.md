---
okf_version: "0.2"
type: Function
title: line_xy
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/line_xy
language: rust
---

# line_xy

## Signature

```rust
fn line_xy(
        sketch: &oxide_sketch::SketchData,
        id: SketchEntityId,
    ) -> Option<((f64, f64), (f64, f64))>
```

## Source
Lines 378–400 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [pick_line_at_for_trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at_for_trim.md) |
| called_by | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
