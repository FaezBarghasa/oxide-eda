---
okf_version: "0.2"
type: Function
title: delete_silk_f
description: v0.18.18 — delete the selected silk-front graphic.
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/delete_silk_f
language: rust
---

# delete_silk_f

v0.18.18 — delete the selected silk-front graphic.

## Signature

```rust
fn delete_silk_f(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.18.18 — delete the selected silk-front graphic.
No-op when nothing is selected. Updates `editor.dirty`
and clears the selection state.

## Source
Lines 93–110 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [adjust_selection_after_remove](/crates/oxide-app/src/library/editor/footprint/state/mod/adjust_selection_after_remove.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
