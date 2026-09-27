---
okf_version: "0.2"
type: Function
title: show_panel
description: "Open `kind`, or reveal it if it is already on screen."
resource: crates/oxide-app/src/app/handlers/dock/panel_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/panel_open/show_panel_1
language: rust
---

# show_panel

Open `kind`, or reveal it if it is already on screen.

## Signature

```rust
pub(crate) fn show_panel(&mut self, kind: PanelKind) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Open `kind`, or reveal it if it is already on screen.

A panel kind exists at most once across the whole app (#641).
It can live in one of three places, and this checks all of
them before placing anything:

- a detached OS window — raise that window,
- somewhere in the dock — reveal it there (`DockArea::show_panel`
handles both docked tabs and floating panels),
- nowhere — dock it at `PanelPosition::default_for(kind)`.

Every open gesture routes through here: the View menu, the
status-bar panel list, an ERC run surfacing its results, TAB
during placement surfacing Properties.

## Source
Lines 24–36 in `crates/oxide-app/src/app/handlers/dock/panel_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_open](/crates/oxide-app/src/app/handlers/dock/panel_open.md) |
| calls | [write_dock_layout](/crates/oxide-app/src/fonts/dock_layout/write_dock_layout.md) |
