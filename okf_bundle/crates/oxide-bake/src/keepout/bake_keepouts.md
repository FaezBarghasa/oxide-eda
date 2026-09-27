---
okf_version: "0.2"
type: Function
title: bake_keepouts
resource: crates/oxide-bake/src/keepout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/keepout/bake_keepouts
language: rust
---

# bake_keepouts

## Signature

```rust
pub fn bake_keepouts(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<FpKeepout>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Source
Lines 32–78 in `crates/oxide-bake/src/keepout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keepout](/crates/oxide-bake/src/keepout.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| calls | [map_kinds](/crates/oxide-bake/src/keepout/map_kinds.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_keepout_multiple_kinds_maps_to_all](/crates/oxide-bake/src/keepout/bake_keepout_multiple_kinds_maps_to_all.md) |
| called_by | [bake_keepout_no_kinds_set_is_all](/crates/oxide-bake/src/keepout/bake_keepout_no_kinds_set_is_all.md) |
| called_by | [bake_keepout_pours_maps_to_copper](/crates/oxide-bake/src/keepout/bake_keepout_pours_maps_to_copper.md) |
| called_by | [bake_keepout_single_kind_routing_maps_to_tracks](/crates/oxide-bake/src/keepout/bake_keepout_single_kind_routing_maps_to_tracks.md) |
