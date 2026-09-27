---
okf_version: "0.2"
type: Function
title: run_polygon_smoke_pass_with
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_polygon_smoke_pass_with
language: rust
---

# run_polygon_smoke_pass_with

## Signature

```rust
fn run_polygon_smoke_pass_with(
    scale_px_per_mm: f32,
    polygons: &[GpuPolygon],
) -> Result<u32, String>
```

## Source
Lines 430–508 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| called_by | [run_polygon_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_polygon_smoke_pass.md) |
| called_by | [polygon_smoke_pass_handles_low_and_high_zoom](/crates/oxide-gfx/src/debug_pass/tests/polygon_smoke_pass_handles_low_and_high_zoom.md) |
| called_by | [polygon_smoke_pass_ignores_degenerate_geometry](/crates/oxide-gfx/src/debug_pass/tests/polygon_smoke_pass_ignores_degenerate_geometry.md) |
