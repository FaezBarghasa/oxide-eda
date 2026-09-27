---
okf_version: "0.2"
type: Function
title: log_debug
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/log_debug
language: rust
---

# log_debug

## Signature

```rust
pub fn log_debug(message: impl AsRef<str>)
```

## Visibility

- `pub`

## Source
Lines 64–66 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [handle_layout_drag_finished](/crates/oxide-app/src/app/handlers/canvas/layout_drag/handle_layout_drag_finished.md) |
| called_by | [handle_layout_drag_started](/crates/oxide-app/src/app/handlers/canvas/layout_drag/handle_layout_drag_started.md) |
| called_by | [handle_dock_floating_layout_message](/crates/oxide-app/src/app/handlers/dock/floating_layout/handle_dock_floating_layout_message.md) |
