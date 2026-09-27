---
okf_version: "0.2"
type: Function
title: fp_editor_set_array_numbering_scheme
description: "v0.23 — Switch numbering scheme. Maps the panel's enum onto"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_array_numbering_scheme
language: rust
---

# fp_editor_set_array_numbering_scheme

v0.23 — Switch numbering scheme. Maps the panel's enum onto

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_array_numbering_scheme(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
        scheme: crate::panels::NumberingSchemeKindUi,
    ) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Switch numbering scheme. Maps the panel's enum onto
[`oxide_sketch::array::NumberingScheme`] using sensible
defaults (1-step LinearIncrement, BGA `A1`-rooted, empty
Explicit list). Existing inner state isn't preserved across
kind flips — switching numbering schemes is a discrete edit.

## Source
Lines 369–405 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
