---
okf_version: "0.2"
type: Function
title: save_entry
description: Save row for the project-root menu — active only when a schematic tab
resource: crates/oxide-app/src/app/view/context_menu/items.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/items/save_entry
language: rust
---

# save_entry

Save row for the project-root menu — active only when a schematic tab

## Signature

```rust
pub(super) fn save_entry(enabled: bool) -> DropdownEntry<Message>
```

## Visibility

- `pub(super)`

## Docstring

Save row for the project-root menu — active only when a schematic tab
is open or the project metadata is dirty. Altium's right-click menus
don't surface keyboard shortcuts, so the shortcut column stays empty
even though Ctrl+S still fires `MenuMessage::Save` globally.

## Source
Lines 78–89 in `crates/oxide-app/src/app/view/context_menu/items.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [items](/crates/oxide-app/src/app/view/context_menu/items.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| called_by | [view_project_tree_context_menu](/crates/oxide-app/src/app/view/context_menu/project_tree/view_project_tree_context_menu.md) |
