---
okf_version: "0.2"
type: Function
title: edit_parameter
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/edit_parameter
language: rust
---

# edit_parameter

## Signature

```rust
fn edit_parameter(editor: &mut crate::app::FootprintEditorState, name: String, expr: String)
```

## Source
Lines 26–34 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/apply.md) |
