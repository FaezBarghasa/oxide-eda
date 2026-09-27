---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-bake/src/mask.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/mask/solve
language: rust
---

# solve

## Signature

```rust
fn solve(sketch: &SketchData) -> FullSolveOutput
```

## Source
Lines 150–154 in `crates/oxide-bake/src/mask.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mask](/crates/oxide-bake/src/mask.md) |
| called_by | [bake_mask_exclude_rectangle](/crates/oxide-bake/src/mask/bake_mask_exclude_rectangle.md) |
| called_by | [bake_mask_opening_rectangle](/crates/oxide-bake/src/mask/bake_mask_opening_rectangle.md) |
| called_by | [bake_paste_aperture_rectangle](/crates/oxide-bake/src/mask/bake_paste_aperture_rectangle.md) |
