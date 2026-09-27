---
okf_version: "0.2"
type: Function
title: handle_footprint_primitive_edit
description: "Footprint-tab branch of [`Self::handle_primitive_editor_event`]."
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/handle_footprint_primitive_edit
language: rust
---

# handle_footprint_primitive_edit

Footprint-tab branch of [`Self::handle_primitive_editor_event`].

## Signature

```rust
impl Oxide { fn handle_footprint_primitive_edit(
        &mut self,
        path: std::path::PathBuf,
        msg: FootprintEditorMsg,
    ) -> Task<Message> }
```

## Docstring

Footprint-tab branch of [`Self::handle_primitive_editor_event`].
Clipboard ops split-borrow `pad_clipboard` alongside the editor;
everything else routes to the standalone footprint editor keyed
by `path`.

## Source
Lines 322–373 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [apply_footprint_clipboard_op](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op.md) |
| calls | [apply_footprint_primitive_edit](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_primitive_edit.md) |
| calls | [read_footprint_filter_presets](/crates/oxide-app/src/fonts/presets/read_footprint_filter_presets.md) |
