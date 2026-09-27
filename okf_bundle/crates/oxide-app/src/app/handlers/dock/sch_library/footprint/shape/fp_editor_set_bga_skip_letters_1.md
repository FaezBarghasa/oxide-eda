---
okf_version: "0.2"
type: Function
title: fp_editor_set_bga_skip_letters
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_bga_skip_letters_1
language: rust
---

# fp_editor_set_bga_skip_letters

## Signature

```rust
pub(crate) fn fp_editor_set_bga_skip_letters(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
        skip_letters: bool,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Source
Lines 407–432 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
