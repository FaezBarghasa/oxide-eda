---
okf_version: "0.2"
type: Function
title: stroke_world_mm
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/stroke_world_mm
language: rust
---

# stroke_world_mm

## Signature

```rust
fn stroke_world_mm(base_width_px_at_100: f32, scale: f32) -> f32
```

## Source
Lines 372–375 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [stroke_px_at_zoom](/crates/oxide-app/src/schematic_runtime/mod/stroke_px_at_zoom.md) |
| called_by | [label_marker_polygon](/crates/oxide-app/src/schematic_runtime/mod/label_marker_polygon.md) |
