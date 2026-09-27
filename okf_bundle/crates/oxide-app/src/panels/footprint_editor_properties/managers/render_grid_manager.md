---
okf_version: "0.2"
type: Function
title: render_grid_manager
description: "v0.18.21 — Grid Manager table. One row per `GridDef`. The active"
resource: crates/oxide-app/src/panels/footprint_editor_properties/managers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/managers/render_grid_manager
language: rust
---

# render_grid_manager

v0.18.21 — Grid Manager table. One row per `GridDef`. The active

## Signature

```rust
pub(super) fn render_grid_manager(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    primary: Color,
    muted: Color,
    border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.18.21 — Grid Manager table. One row per `GridDef`. The active
row is highlighted; clicking another row activates it (mirrors its
step / display style onto `snap_options`). The footer's Add /
Properties / Delete operate on the active row.

## Source
Lines 13–153 in `crates/oxide-app/src/panels/footprint_editor_properties/managers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [managers](/crates/oxide-app/src/panels/footprint_editor_properties/managers.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
