---
okf_version: "0.2"
type: Function
title: fill_color_for
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/fill_color_for
language: rust
---

# fill_color_for

## Signature

```rust
fn fill_color_for(
    fill: FillType,
    stroke_color: &Option<oxide_types::schematic::StrokeColor>,
    colors: &CanvasColors,
) -> Option<Color>
```

## Source
Lines 735–745 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [resolve_stroke_color](/crates/oxide-app/src/schematic_runtime/mod/resolve_stroke_color.md) |
| calls | [to_iced](/crates/oxide-app/src/schematic_runtime/mod/to_iced.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
