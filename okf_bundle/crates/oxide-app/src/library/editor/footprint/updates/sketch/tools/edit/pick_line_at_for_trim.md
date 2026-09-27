---
okf_version: "0.2"
type: Function
title: pick_line_at_for_trim
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at_for_trim
language: rust
---

# pick_line_at_for_trim

## Signature

```rust
fn pick_line_at_for_trim(
        sketch: &oxide_sketch::SketchData,
        x: f64,
        y: f64,
    ) -> Option<SketchEntityId>
```

## Source
Lines 401–429 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [line_xy](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/line_xy.md) |
| called_by | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
