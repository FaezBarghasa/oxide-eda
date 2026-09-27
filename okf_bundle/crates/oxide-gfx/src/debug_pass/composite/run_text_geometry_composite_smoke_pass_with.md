---
okf_version: "0.2"
type: Function
title: run_text_geometry_composite_smoke_pass_with
resource: crates/oxide-gfx/src/debug_pass/composite.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/debug_pass/composite/run_text_geometry_composite_smoke_pass_with
language: rust
---

# run_text_geometry_composite_smoke_pass_with

## Signature

```rust
fn run_text_geometry_composite_smoke_pass_with(
    scale_px_per_mm: f32,
    polygons: &[GpuPolygon],
    texts: &[TextItem],
) -> Result<CompositeSmokeReport, String>
```

## Source
Lines 5–117 in `crates/oxide-gfx/src/debug_pass/composite.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [composite](/crates/oxide-gfx/src/debug_pass/composite.md) |
| called_by | [run_text_geometry_composite_smoke_pass](/crates/oxide-gfx/src/debug_pass/composite/run_text_geometry_composite_smoke_pass.md) |
