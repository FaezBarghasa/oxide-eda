---
okf_version: "0.2"
type: Function
title: apply_footprint_clipboard_op
description: v0.26-E — apply Cut / Copy / Paste against the document-level
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op
language: rust
---

# apply_footprint_clipboard_op

v0.26-E — apply Cut / Copy / Paste against the document-level

## Signature

```rust
pub(crate) fn apply_footprint_clipboard_op(
    editor: &mut crate::app::FootprintEditorState,
    clipboard: &mut Option<crate::library::editor::footprint::state::EditorPad>,
    msg: &FootprintEditorMsg,
)
```

## Visibility

- `pub(crate)`

## Docstring

v0.26-E — apply Cut / Copy / Paste against the document-level
`pad_clipboard`. Split-borrowed at the call site so both the
editor and the clipboard slot are mutable.

Behaviour:
- **Copy**: clones the selected pad into the clipboard. No-op
when nothing is selected.
- **Cut**: Copy + delete; mirrors into the sketch + invalidates
the canvas cache.
- **Paste**: places a clone of the clipboard pad at the cursor
(or `original.position + (1mm, 1mm)` if cursor is unknown),
picks a free designator (max + 1), pre-computes a fresh
`sketch_entity_id` so the new pad mirrors into the sketch on
its first edit, and selects the new pad post-paste.

## Source
Lines 189–286 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
| calls | [sketch_is_authored](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sketch_is_authored.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [handle_footprint_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit.md) |
