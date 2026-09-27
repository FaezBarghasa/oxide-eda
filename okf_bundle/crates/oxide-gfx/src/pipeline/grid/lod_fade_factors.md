---
okf_version: "0.2"
type: Function
title: lod_fade_factors
resource: crates/oxide-gfx/src/pipeline/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/grid/lod_fade_factors
language: rust
---

# lod_fade_factors

## Signature

```rust
pub fn lod_fade_factors(mm_per_px: f32) -> GridLodFactors
```

## Visibility

- `pub`

## Source
Lines 27–38 in `crates/oxide-gfx/src/pipeline/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-gfx/src/pipeline/grid.md) |
| calls | [smoothstep](/crates/oxide-gfx/src/pipeline/grid/smoothstep.md) |
| called_by | [run_grid_smoke_pass_with](/crates/oxide-gfx/src/debug_pass/mod/run_grid_smoke_pass_with.md) |
| called_by | [grid_lod_fade_is_density_aware](/crates/oxide-gfx/src/pipeline/grid/grid_lod_fade_is_density_aware.md) |
| called_by | [grid_lod_fade_stays_in_unit_range](/crates/oxide-gfx/src/pipeline/grid/grid_lod_fade_stays_in_unit_range.md) |
