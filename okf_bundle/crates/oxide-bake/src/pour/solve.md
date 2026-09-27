---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-bake/src/pour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/pour/solve
language: rust
---

# solve

## Signature

```rust
fn solve(sketch: &SketchData) -> FullSolveOutput
```

## Source
Lines 168–172 in `crates/oxide-bake/src/pour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pour](/crates/oxide-bake/src/pour.md) |
| called_by | [bake_pour_outline_fill_maps_to_none](/crates/oxide-bake/src/pour/bake_pour_outline_fill_maps_to_none.md) |
| called_by | [bake_pour_records_metadata](/crates/oxide-bake/src/pour/bake_pour_records_metadata.md) |
| called_by | [bake_pour_thermal_disabled_maps_to_direct](/crates/oxide-bake/src/pour/bake_pour_thermal_disabled_maps_to_direct.md) |
