---
okf_version: "0.2"
type: Function
title: view_grid_properties_dialog
description: v0.18.11 — Cartesian Grid Editor modal (Ctrl+G in a footprint
resource: crates/oxide-app/src/app/view/dialogs/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/project/view_grid_properties_dialog_1
language: rust
---

# view_grid_properties_dialog

v0.18.11 — Cartesian Grid Editor modal (Ctrl+G in a footprint

## Signature

```rust
pub(in crate::app::view) fn view_grid_properties_dialog(&self) -> Element<'_, Message>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.18.11 — Cartesian Grid Editor modal (Ctrl+G in a footprint
editor). Mirrors Altium's "Cartesian Grid Editor [mm]" with a
stripped-down field set: Step X / Step Y + link toggle, plus
OK / Cancel. Display style + multiplier + per-grid-color land
in v0.18.11.x as the underlying canvas/grid system grows them.

## Source
Lines 395–404 in `crates/oxide-app/src/app/view/dialogs/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/src/app/view/dialogs/project.md) |
| calls | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
