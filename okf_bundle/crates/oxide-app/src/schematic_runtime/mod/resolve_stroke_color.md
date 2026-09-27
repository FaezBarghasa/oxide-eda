---
okf_version: "0.2"
type: Function
title: resolve_stroke_color
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/resolve_stroke_color
language: rust
---

# resolve_stroke_color

## Signature

```rust
fn resolve_stroke_color(
    stroke_color: &Option<oxide_types::schematic::StrokeColor>,
    fallback: Color,
) -> Color
```

## Source
Lines 726–733 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [fill_color_for](/crates/oxide-app/src/schematic_runtime/mod/fill_color_for.md) |
| called_by | [build_renderer_snapshot](/crates/oxide-app/src/schematic_runtime/snapshot/build_renderer_snapshot.md) |
