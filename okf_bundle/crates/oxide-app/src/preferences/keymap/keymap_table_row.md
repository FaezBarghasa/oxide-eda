---
okf_version: "0.2"
type: Function
title: keymap_table_row
description: Render one shortcut table row. Extracted so the grouped view can build
resource: crates/oxide-app/src/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/preferences/keymap/keymap_table_row
language: rust
---

# keymap_table_row

Render one shortcut table row. Extracted so the grouped view can build

## Signature

```rust
fn keymap_table_row(
    row_model: &crate::keymap::KeymapEditorRow,
    conflicts: &[crate::keymap::BindingConflict],
    active_profile_is_custom: bool,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Docstring

Render one shortcut table row. Extracted so the grouped view can build
rows per [`crate::keymap::CommandGroup`] without duplicating the cell
layout. Text that sits directly on the (theme-neutral) modal surface
keeps the shared muted / primary constants; the trigger chip and Edit
button pull their colours from the active theme.

## Source
Lines 217–314 in `crates/oxide-app/src/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/preferences/keymap.md) |
| calls | [shortcut_chip](/crates/oxide-app/src/preferences/keymap/shortcut_chip.md) |
| called_by | [content_keyboard_shortcuts](/crates/oxide-app/src/preferences/keymap/content_keyboard_shortcuts.md) |
