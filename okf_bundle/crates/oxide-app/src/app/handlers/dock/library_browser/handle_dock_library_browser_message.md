---
okf_version: "0.2"
type: Function
title: handle_dock_library_browser_message
resource: crates/oxide-app/src/app/handlers/dock/library_browser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/library_browser/handle_dock_library_browser_message
language: rust
---

# handle_dock_library_browser_message

## Signature

```rust
impl Oxide { pub(super) fn handle_dock_library_browser_message(
        &mut self,
        panel_msg: &crate::panels::PanelMsg,
    ) -> bool }
```

## Visibility

- `pub(super)`

## Source
Lines 6–37 in `crates/oxide-app/src/app/handlers/dock/library_browser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser](/crates/oxide-app/src/app/handlers/dock/library_browser.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
