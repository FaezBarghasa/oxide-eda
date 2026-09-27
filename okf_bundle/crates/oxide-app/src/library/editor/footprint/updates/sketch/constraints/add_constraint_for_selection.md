---
okf_version: "0.2"
type: Function
title: add_constraint_for_selection
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection
language: rust
---

# add_constraint_for_selection

## Signature

```rust
fn add_constraint_for_selection(
    editor: &mut crate::app::FootprintEditorState,
    tag: crate::library::messages::SketchConstraintTag,
)
```

## Source
Lines 36–248 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [dim_input_error](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/dim_input_error.md) |
| calls | [report_constraint_not_added](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/report_constraint_not_added.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/apply.md) |
