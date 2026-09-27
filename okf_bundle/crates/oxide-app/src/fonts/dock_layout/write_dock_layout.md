---
okf_version: "0.2"
type: Function
title: write_dock_layout
description: Persist the list of dock panels per region + their active index
resource: crates/oxide-app/src/fonts/dock_layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:28Z"
concept_id: crates/oxide-app/src/fonts/dock_layout/write_dock_layout
language: rust
---

# write_dock_layout

Persist the list of dock panels per region + their active index

## Signature

```rust
pub fn write_dock_layout(dock: &crate::dock::DockArea)
```

## Visibility

- `pub`

## Docstring

Persist the list of dock panels per region + their active index
so the next session reopens with the same layout.

## Source
Lines 7–33 in `crates/oxide-app/src/fonts/dock_layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dock_layout](/crates/oxide-app/src/fonts/dock_layout.md) |
| calls | [panel_kind_key](/crates/oxide-app/src/fonts/dock_layout/panel_kind_key.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [dispatch_window_message](/crates/oxide-app/src/app/dispatch/mod/dispatch_window_message.md) |
| called_by | [handle_dock_message](/crates/oxide-app/src/app/handlers/dock/mod/handle_dock_message.md) |
| called_by | [show_panel](/crates/oxide-app/src/app/handlers/dock/panel_open/show_panel.md) |
