---
okf_version: "0.2"
type: Function
title: fp_editor_delete_array
description: v0.23 — Delete an array. The source entity stays in the sketch
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_delete_array_1
language: rust
---

# fp_editor_delete_array

v0.23 — Delete an array. The source entity stays in the sketch

## Signature

```rust
pub(crate) fn fp_editor_delete_array(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Delete an array. The source entity stays in the sketch
— only the array record is removed, so existing constraints on
the source survive intact.

## Source
Lines 506–524 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
