---
okf_version: "0.2"
type: Function
title: handle_dock_floating_layout_message
resource: crates/oxide-app/src/app/handlers/dock/floating_layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/floating_layout/handle_dock_floating_layout_message
language: rust
---

# handle_dock_floating_layout_message

## Signature

```rust
impl Oxide { pub(super) fn handle_dock_floating_layout_message(
        &mut self,
        dock_message: &DockMessage,
    ) -> bool }
```

## Visibility

- `pub(super)`

## Source
Lines 6–80 in `crates/oxide-app/src/app/handlers/dock/floating_layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [floating_layout](/crates/oxide-app/src/app/handlers/dock/floating_layout.md) |
| calls | [log_debug](/crates/oxide-app/src/diagnostics/log_debug.md) |
