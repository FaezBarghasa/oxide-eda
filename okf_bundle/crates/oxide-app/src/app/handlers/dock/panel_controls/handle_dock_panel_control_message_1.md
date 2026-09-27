---
okf_version: "0.2"
type: Function
title: handle_dock_panel_control_message
description: "Returns `None` when the message isn't a panel-control message"
resource: crates/oxide-app/src/app/handlers/dock/panel_controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:39:58Z"
concept_id: crates/oxide-app/src/app/handlers/dock/panel_controls/handle_dock_panel_control_message_1
language: rust
---

# handle_dock_panel_control_message

Returns `None` when the message isn't a panel-control message

## Signature

```rust
pub(super) fn handle_dock_panel_control_message(
        &mut self,
        panel_msg: &crate::panels::PanelMsg,
    ) -> Option<Task<Message>>
```

## Visibility

- `pub(super)`

## Docstring

Returns `None` when the message isn't a panel-control message
(so the caller can fall through to the next dock handler), or
`Some(task)` when handled — the task carries any follow-up work
from a re-entrant `self.update(...)` so it isn't dropped.

## Source
Lines 25–555 in `crates/oxide-app/src/app/handlers/dock/panel_controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_controls](/crates/oxide-app/src/app/handlers/dock/panel_controls.md) |
| calls | [write_component_filter](/crates/oxide-app/src/fonts/misc/write_component_filter.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [write_grid_size_mm_pref](/crates/oxide-app/src/fonts/mod/write_grid_size_mm_pref.md) |
| calls | [write_ui_font_pref](/crates/oxide-app/src/fonts/mod/write_ui_font_pref.md) |
