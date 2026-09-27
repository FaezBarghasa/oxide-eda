---
okf_version: "0.2"
type: Function
title: submenu_header_entry
description: "A submenu-launcher row — toggles `open_submenu` on click. Not"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod/submenu_header_entry
language: rust
---

# submenu_header_entry

A submenu-launcher row — toggles `open_submenu` on click. Not

## Signature

```rust
fn submenu_header_entry(
    id: &'static str,
    label: &'static str,
    is_open: bool,
    path: &Path,
) -> DropdownEntry<LibraryMessage>
```

## Docstring

A submenu-launcher row — toggles `open_submenu` on click. Not
wrapped in `ContextMenuAction`: opening/closing a submenu must not
close the whole popover.

## Source
Lines 112–125 in `crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod.md) |
| calls | [submenu_msg_for_id](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/submenu_msg_for_id.md) |
| calls | [wrap](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/wrap.md) |
| called_by | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
