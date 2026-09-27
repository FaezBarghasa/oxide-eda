---
okf_version: "0.2"
type: Function
title: run_grid_smoke_pass_with
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_grid_smoke_pass_with
language: rust
---

# run_grid_smoke_pass_with

## Signature

```rust
fn run_grid_smoke_pass_with(scale_px_per_mm: f32) -> Result<GridSmokeReport, String>
```

## Source
Lines 521–602 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| calls | [lod_fade_factors](/crates/oxide-gfx/src/pipeline/grid/lod_fade_factors.md) |
| called_by | [run_grid_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_grid_smoke_pass.md) |
| called_by | [grid_smoke_pass_lod_changes_with_zoom](/crates/oxide-gfx/src/debug_pass/tests/grid_smoke_pass_lod_changes_with_zoom.md) |
