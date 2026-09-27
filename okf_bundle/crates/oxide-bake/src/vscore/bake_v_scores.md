---
okf_version: "0.2"
type: Function
title: bake_v_scores
resource: crates/oxide-bake/src/vscore.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bake/src/vscore/bake_v_scores
language: rust
---

# bake_v_scores

## Signature

```rust
pub fn bake_v_scores(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    out: &mut Vec<FpVScore>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Source
Lines 35–115 in `crates/oxide-bake/src/vscore.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vscore](/crates/oxide-bake/src/vscore.md) |
| calls | [build_ctx](/crates/oxide-bake/src/vscore/build_ctx.md) |
| calls | [eval_dimensionless](/crates/oxide-bake/src/vscore/eval_dimensionless.md) |
| calls | [opt_eval_mm](/crates/oxide-bake/src/vscore/opt_eval_mm.md) |
| calls | [map_side](/crates/oxide-bake/src/vscore/map_side.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_v_score_arc_skipped](/crates/oxide-bake/src/vscore/bake_v_score_arc_skipped.md) |
| called_by | [bake_v_score_clamps_depth_fraction](/crates/oxide-bake/src/vscore/bake_v_score_clamps_depth_fraction.md) |
| called_by | [bake_v_score_horizontal_line](/crates/oxide-bake/src/vscore/bake_v_score_horizontal_line.md) |
| called_by | [bake_v_score_min_web_baked](/crates/oxide-bake/src/vscore/bake_v_score_min_web_baked.md) |
