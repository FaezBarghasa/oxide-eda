---
okf_version: "0.2"
type: Function
title: handle_active_bar_placement_preset
resource: crates/oxide-app/src/app/handlers/active_bar/placement_presets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/active_bar/placement_presets/handle_active_bar_placement_preset_1
language: rust
---

# handle_active_bar_placement_preset

## Signature

```rust
pub(super) fn handle_active_bar_placement_preset(
        &mut self,
        action: crate::active_bar::ActiveBarAction,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Source
Lines 6–258 in `crates/oxide-app/src/app/handlers/active_bar/placement_presets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement_presets](/crates/oxide-app/src/app/handlers/active_bar/placement_presets.md) |
| calls | [Tool](/crates/oxide-app/src/app/documents/Tool.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
