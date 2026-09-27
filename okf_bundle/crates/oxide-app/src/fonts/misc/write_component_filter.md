---
okf_version: "0.2"
type: Function
title: write_component_filter
description: Persist the Components-panel filter without clobbering other keys.
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/write_component_filter
language: rust
---

# write_component_filter

Persist the Components-panel filter without clobbering other keys.

## Signature

```rust
pub fn write_component_filter(query: &str)
```

## Visibility

- `pub`

## Docstring

Persist the Components-panel filter without clobbering other keys.

## Source
Lines 53–60 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| called_by | [handle_dock_panel_control_message](/crates/oxide-app/src/app/handlers/dock/panel_controls/handle_dock_panel_control_message.md) |
