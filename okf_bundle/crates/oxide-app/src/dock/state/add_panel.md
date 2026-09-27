---
okf_version: "0.2"
type: Function
title: add_panel
description: "Dock `kind` at `position`, but only when it is not already open"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/add_panel
language: rust
---

# add_panel

Dock `kind` at `position`, but only when it is not already open

## Signature

```rust
impl DockArea { pub fn add_panel(&mut self, position: PanelPosition, kind: PanelKind) }
```

## Visibility

- `pub`

## Docstring

Dock `kind` at `position`, but only when it is not already open
somewhere in the dock. Placement only: an existing panel is
left exactly where and how it is, active tab included.

The guard spans the whole dock, not just `position` (#641).
Per-region dedupe let one kind be docked up to three times, and
`fonts::write_dock_layout` then persisted the duplicates. The
same global guard is what de-duplicates a `prefs.json` written
before this fix: `fonts::read_dock_layout` replays a saved
layout through here region by region, so the second and third
copies are dropped on load.

## Source
Lines 71–76 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
