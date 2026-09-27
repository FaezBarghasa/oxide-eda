---
okf_version: "0.2"
type: Function
title: fp_editor_toggle_array_instance
description: "v0.23 — Toggle a single `(i, j)` instance in an array's"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_toggle_array_instance
language: rust
---

# fp_editor_toggle_array_instance

v0.23 — Toggle a single `(i, j)` instance in an array's

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_toggle_array_instance(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
        i: u32,
        j: u32,
        value: bool,
    ) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Toggle a single `(i, j)` instance in an array's
per-instance suppression list. `value=true` enables the
instance (removes the entry); `value=false` suppresses it
(adds the entry, deduplicated). Polar arrays set `j = 0`.

When the suppression list grows from empty, the array gains a
fresh `GridDepopulation { mask_expr: "", suppressed_instances }`.
When the list returns to empty AND the existing `mask_expr` is
blank, the depopulation is removed entirely so the array
returns to its parametric-only state.

## Source
Lines 554–605 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
