---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/apply
language: rust
---

# apply

## Signature

```rust
pub(in crate::library::editor::footprint::updates) fn apply(
    editor: &mut crate::app::FootprintEditorState,
    msg: FootprintEditorMsg,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::updates)`

## Source
Lines 9–24 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.md) |
| calls | [edit_parameter](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/edit_parameter.md) |
| calls | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
