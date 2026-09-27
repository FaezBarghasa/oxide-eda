---
okf_version: "0.2"
type: Function
title: content_keyboard_shortcuts
resource: crates/oxide-app/src/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/preferences/keymap/content_keyboard_shortcuts
language: rust
---

# content_keyboard_shortcuts

## Signature

```rust
pub(super) fn content_keyboard_shortcuts(
    editor: &'a crate::keymap::KeymapEditorModel,
    status: &'a str,
    load_error: Option<&'a str>,
    backup: Option<&'a str>,
    search: &'a str,
    recorder: Option<&'a crate::app::KeymapRecorderState>,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 10–210 in `crates/oxide-app/src/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/preferences/keymap.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [keymap_recorder_control](/crates/oxide-app/src/preferences/keymap/keymap_recorder_control.md) |
| calls | [h_sep](/crates/oxide-app/src/preferences/widgets/h_sep.md) |
| calls | [keymap_table_row](/crates/oxide-app/src/preferences/keymap/keymap_table_row.md) |
| called_by | [build_content](/crates/oxide-app/src/preferences/mod/build_content.md) |
