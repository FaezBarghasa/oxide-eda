---
okf_version: "0.2"
type: Class
title: PanelSite
description: Where a panel kind currently lives inside the dock.
resource: crates/oxide-app/src/dock/types.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/types/PanelSite
language: rust
---

# PanelSite

Where a panel kind currently lives inside the dock.

## Signature

```rust
pub enum PanelSite
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Where a panel kind currently lives inside the dock.

A kind is expected to occupy at most one of these — see
`DockArea::locate`. Detached OS windows are a fourth home the dock
cannot see; `Oxide::show_panel` checks `ui_state.windows` for those.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 69–74 in `crates/oxide-app/src/dock/types.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [types](/crates/oxide-app/src/dock/types.md) |
