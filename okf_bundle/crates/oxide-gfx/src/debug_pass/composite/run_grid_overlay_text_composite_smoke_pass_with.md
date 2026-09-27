---
okf_version: "0.2"
type: Function
title: run_grid_overlay_text_composite_smoke_pass_with
resource: crates/oxide-gfx/src/debug_pass/composite.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/debug_pass/composite/run_grid_overlay_text_composite_smoke_pass_with
language: rust
---

# run_grid_overlay_text_composite_smoke_pass_with

## Signature

```rust
pub(super) fn run_grid_overlay_text_composite_smoke_pass_with(
    scale_px_per_mm: f32,
    grid_enabled: bool,
    overlay_enabled: bool,
    text_enabled: bool,
    polygons: &[GpuPolygon],
    overlay_lines: &[LineSegment],
    texts: &[TextItem],
) -> Result<OverlayCompositeSmokeReport, String>
```

## Visibility

- `pub(super)`

## Source
Lines 142–282 in `crates/oxide-gfx/src/debug_pass/composite.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [composite](/crates/oxide-gfx/src/debug_pass/composite.md) |
| called_by | [run_grid_overlay_text_composite_smoke_pass](/crates/oxide-gfx/src/debug_pass/composite/run_grid_overlay_text_composite_smoke_pass.md) |
| called_by | [grid_overlay_toggles_do_not_change_geometry_draw_work](/crates/oxide-gfx/src/debug_pass/tests/grid_overlay_toggles_do_not_change_geometry_draw_work.md) |
