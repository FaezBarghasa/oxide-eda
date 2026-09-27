---
okf_version: "0.2"
type: Function
title: fp_editor_edit_array_param
description: "v0.23 — Pattern sub-form text-input edit. Walks `sketch.arrays`"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_edit_array_param_1
language: rust
---

# fp_editor_edit_array_param

v0.23 — Pattern sub-form text-input edit. Walks `sketch.arrays`

## Signature

```rust
pub(crate) fn fp_editor_edit_array_param(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
        field: crate::panels::ArrayParamField,
        value: String,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Pattern sub-form text-input edit. Walks `sketch.arrays`
to find the array, mutates the field identified by
`ArrayParamField`, then runs `SketchEdit::ForceRebuild` so the
bake re-expands. `MaskExpr` with an empty value clears the
depopulation entirely (to avoid leaving a `mask_expr=""` orphan
that blocks re-enabling instances later).

## Source
Lines 279–362 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
