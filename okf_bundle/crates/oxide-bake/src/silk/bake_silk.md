---
okf_version: "0.2"
type: Function
title: bake_silk
description: Bake every SilkAttr-tagged non-construction entity into an
resource: crates/oxide-bake/src/silk.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/silk/bake_silk
language: rust
---

# bake_silk

Bake every SilkAttr-tagged non-construction entity into an

## Signature

```rust
pub fn bake_silk(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    silk_f: &mut Vec<FpGraphic>,
    silk_b: &mut Vec<FpGraphic>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake every SilkAttr-tagged non-construction entity into an
`FpGraphic` on `silk_f` or `silk_b`. Both Vecs are passed by mut-ref
and appended to (the caller decides whether to clear them first).

Returns `Ok(())` even when individual entities skip — those are
reported via the `warnings` Vec instead.

## Source
Lines 42–82 in `crates/oxide-bake/src/silk.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [silk](/crates/oxide-bake/src/silk.md) |
| calls | [entity_to_graphic](/crates/oxide-bake/src/silk/entity_to_graphic.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_silk_circle_to_bottom_silk](/crates/oxide-bake/src/silk/bake_silk_circle_to_bottom_silk.md) |
| called_by | [bake_silk_construction_skipped](/crates/oxide-bake/src/silk/bake_silk_construction_skipped.md) |
| called_by | [bake_silk_line_to_top_silk](/crates/oxide-bake/src/silk/bake_silk_line_to_top_silk.md) |
| called_by | [bake_silk_wrong_layer_warns](/crates/oxide-bake/src/silk/bake_silk_wrong_layer_warns.md) |
