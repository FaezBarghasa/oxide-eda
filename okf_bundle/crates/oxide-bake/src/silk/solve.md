---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-bake/src/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/silk/solve
language: rust
---

# solve

## Signature

```rust
fn solve(sketch: &SketchData) -> FullSolveOutput
```

## Source
Lines 176–180 in `crates/oxide-bake/src/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-bake/src/silk.md) |
| called_by | [bake_silk_circle_to_bottom_silk](/crates/oxide-bake/src/silk/bake_silk_circle_to_bottom_silk.md) |
| called_by | [bake_silk_construction_skipped](/crates/oxide-bake/src/silk/bake_silk_construction_skipped.md) |
| called_by | [bake_silk_line_to_top_silk](/crates/oxide-bake/src/silk/bake_silk_line_to_top_silk.md) |
| called_by | [bake_silk_wrong_layer_warns](/crates/oxide-bake/src/silk/bake_silk_wrong_layer_warns.md) |
