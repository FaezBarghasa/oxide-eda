---
okf_version: "0.2"
type: Function
title: detached_panel_window
description: "The OS window hosting `kind` as a detached panel, if any."
resource: crates/oxide-app/src/app/handlers/dock/panel_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/panel_open/detached_panel_window_1
language: rust
---

# detached_panel_window

The OS window hosting `kind` as a detached panel, if any.

## Signature

```rust
fn detached_panel_window(&self, kind: PanelKind) -> Option<iced::window::Id>
```

## Docstring

The OS window hosting `kind` as a detached panel, if any.

## Source
Lines 39–47 in `crates/oxide-app/src/app/handlers/dock/panel_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_open](/crates/oxide-app/src/app/handlers/dock/panel_open.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
