---
okf_version: "0.2"
type: Function
title: fp_editor_set_bga_start_row
description: "v0.25 polish — set BGA `start_row` letter. Empty / non-letter"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_start_row_1
language: rust
---

# fp_editor_set_bga_start_row

v0.25 polish — set BGA `start_row` letter. Empty / non-letter

## Signature

```rust
pub(crate) fn fp_editor_set_bga_start_row(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
        value: String,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.25 polish — set BGA `start_row` letter. Empty / non-letter
input no-ops; multi-char takes the first letter; uppercased
before storage. Letters I/O/Q/S/X/Z are valid starts even when
`skip_letters = true` (the skip applies to the row alphabet,
not the start point).

## Source
Lines 439–469 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
