---
okf_version: "0.2"
type: Function
title: engine_for_window
description: "Per-window engine lookup. Main window → the active tab's engine"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/engine_for_window
language: rust
---

# engine_for_window

Per-window engine lookup. Main window → the active tab's engine

## Signature

```rust
impl DocumentState { pub fn engine_for_window(
        &self,
        window_id: iced::window::Id,
        ui: &UiState,
    ) -> Option<&oxide_engine::Engine> }
```

## Visibility

- `pub`

## Docstring

Per-window engine lookup. Main window → the active tab's engine
(same as `active_engine`). Undocked tab windows → the engine for
the path the window was opened on. All schematic engines live in
`self.engines`, so every window resolves with a single HashMap
lookup.

## Source
Lines 748–762 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
