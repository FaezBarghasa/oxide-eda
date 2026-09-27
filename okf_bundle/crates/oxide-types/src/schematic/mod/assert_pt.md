---
okf_version: "0.2"
type: Function
title: assert_pt
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/assert_pt
language: rust
---

# assert_pt

## Signature

```rust
fn assert_pt(got: Point, ex: f64, ey: f64)
```

## Source
Lines 773–780 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
| called_by | [identity_flips_library_y_up_to_schematic_y_down](/crates/oxide-types/src/schematic/mod/identity_flips_library_y_up_to_schematic_y_down.md) |
| called_by | [mirror_x_flips_the_y_output_mirror_y_flips_the_x](/crates/oxide-types/src/schematic/mod/mirror_x_flips_the_y_output_mirror_y_flips_the_x.md) |
| called_by | [origin_translates_the_result](/crates/oxide-types/src/schematic/mod/origin_translates_the_result.md) |
| called_by | [rotation_90_turns_the_axes](/crates/oxide-types/src/schematic/mod/rotation_90_turns_the_axes.md) |
| called_by | [rotation_and_mirror_compose](/crates/oxide-types/src/schematic/mod/rotation_and_mirror_compose.md) |
