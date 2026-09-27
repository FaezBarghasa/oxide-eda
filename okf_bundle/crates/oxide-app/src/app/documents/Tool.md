---
okf_version: "0.2"
type: Class
title: Tool
description: "[derive(Debug, Clone, Copy, PartialEq, Eq)]"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/Tool
language: rust
---

# Tool

[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Signature

```rust
pub enum Tool
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 641–657 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
| called_by | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
| called_by | [escape_tool_reset](/crates/oxide-app/src/app/dispatch/escape/escape_tool_reset.md) |
| called_by | [handle_active_bar_action](/crates/oxide-app/src/app/handlers/active_bar/action_groups/handle_active_bar_action.md) |
| called_by | [handle_active_bar_placement_preset](/crates/oxide-app/src/app/handlers/active_bar/placement_presets/handle_active_bar_placement_preset.md) |
| called_by | [set_pending_power_port](/crates/oxide-app/src/app/handlers/active_bar/placement_presets/set_pending_power_port.md) |
| called_by | [update_right_released](/crates/oxide-app/src/canvas/input/pointer/update_right_released.md) |
