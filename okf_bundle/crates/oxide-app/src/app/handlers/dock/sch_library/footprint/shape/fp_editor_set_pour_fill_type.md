---
okf_version: "0.2"
type: Function
title: fp_editor_set_pour_fill_type
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_set_pour_fill_type
language: rust
---

# fp_editor_set_pour_fill_type

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_pour_fill_type(
        &mut self,
        id: oxide_sketch::id::SketchEntityId,
        value: oxide_sketch::attr::PourFillType,
    ) -> bool }
```

## Visibility

- `pub(crate)`

## Source
Lines 43–65 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
