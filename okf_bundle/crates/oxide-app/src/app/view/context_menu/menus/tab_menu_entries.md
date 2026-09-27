---
okf_version: "0.2"
type: Function
title: tab_menu_entries
description: "Pure tab-menu data builder. The per-tab \"Close [title]\" row carries the"
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/tab_menu_entries
language: rust
---

# tab_menu_entries

Pure tab-menu data builder. The per-tab "Close [title]" row carries the

## Signature

```rust
pub(super) fn tab_menu_entries(
    title: &str,
    tab_idx: usize,
    total_tabs: usize,
    already_undocked: bool,
) -> Vec<DropdownEntry<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Pure tab-menu data builder. The per-tab "Close [title]" row carries the
live tab title; the bulk-close rows are gated on whether they'd be
no-ops (single tab open → no "others" to close); "Open In New Window"
greys out when the tab already lives in its own OS window.

## Source
Lines 369–413 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
| calls | [dd_msg](/crates/oxide-app/src/app/view/context_menu/items/dd_msg.md) |
| calls | [dd_disabled](/crates/oxide-app/src/app/view/context_menu/items/dd_disabled.md) |
| called_by | [view_tab_context_menu](/crates/oxide-app/src/app/view/context_menu/menus/view_tab_context_menu.md) |
| called_by | [tab_menu_gates_bulk_close_and_undock](/crates/oxide-app/src/app/view/context_menu/tests/tab_menu_gates_bulk_close_and_undock.md) |
