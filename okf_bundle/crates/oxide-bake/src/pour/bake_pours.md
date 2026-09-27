---
okf_version: "0.2"
type: Function
title: bake_pours
description: "Bake every PourAttr-tagged closed profile into `out`."
resource: crates/oxide-bake/src/pour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/pour/bake_pours
language: rust
---

# bake_pours

Bake every PourAttr-tagged closed profile into `out`.

## Signature

```rust
pub fn bake_pours(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    params_canonical: &HashMap<String, f64>,
    out: &mut Vec<FpPour>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake every PourAttr-tagged closed profile into `out`.

## Source
Lines 29–114 in `crates/oxide-bake/src/pour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pour](/crates/oxide-bake/src/pour.md) |
| calls | [build_ctx](/crates/oxide-bake/src/pour/build_ctx.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| calls | [opt_eval_mm](/crates/oxide-bake/src/pour/opt_eval_mm.md) |
| calls | [NetRef](/crates/oxide-library/src/primitive/footprint/mod/NetRef.md) |
| calls | [map_fill](/crates/oxide-bake/src/pour/map_fill.md) |
| calls | [map_thermal](/crates/oxide-bake/src/pour/map_thermal.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_pour_outline_fill_maps_to_none](/crates/oxide-bake/src/pour/bake_pour_outline_fill_maps_to_none.md) |
| called_by | [bake_pour_records_metadata](/crates/oxide-bake/src/pour/bake_pour_records_metadata.md) |
| called_by | [bake_pour_thermal_disabled_maps_to_direct](/crates/oxide-bake/src/pour/bake_pour_thermal_disabled_maps_to_direct.md) |
