---
okf_version: "0.2"
type: Function
title: active_bar_rotate_selection
description: v0.28 — Rotate / Flip / Align-to-Grid / Move-origin-to-Grid act on the
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_rotate_selection
language: rust
---

# active_bar_rotate_selection

v0.28 — Rotate / Flip / Align-to-Grid / Move-origin-to-Grid act on the

## Signature

```rust
fn active_bar_rotate_selection(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

v0.28 — Rotate / Flip / Align-to-Grid / Move-origin-to-Grid act on the
WHOLE selection. All three used to read `state.selected_pad` alone and
silently transform one pad out of N; a partial flip in particular
leaves mixed F./B. layers, which is a fab error. #146 put all three on
`mutates_footprint_state`'s exemption list, so
`apply_footprint_primitive_edit` does NOT blanket-push for them: each
snapshots here itself — exactly once, and only when the selection is
non-empty, so a no-op transform never stacks undo history or dirties
the document.

## Source
Lines 169–193 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| calls | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| calls | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
