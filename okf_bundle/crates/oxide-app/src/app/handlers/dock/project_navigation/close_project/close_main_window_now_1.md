---
okf_version: "0.2"
type: Function
title: close_main_window_now
description: "Actually close the main window. In `iced::daemon` this fires a"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/close_main_window_now_1
language: rust
---

# close_main_window_now

Actually close the main window. In `iced::daemon` this fires a

## Signature

```rust
fn close_main_window_now(&self) -> Task<Message>
```

## Docstring

Actually close the main window. In `iced::daemon` this fires a
`Closed` event → `SecondaryWindowClosed(main)` → `iced::exit()`,
so all shutdown bookkeeping stays on the existing path.

## Source
Lines 307–312 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
