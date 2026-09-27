---
okf_version: "0.2"
type: Function
title: render_snapping_mode_row
description: "v0.18.14.3 — Altium \"Snapping\" 3-segment toggle. `All Layers` is"
resource: crates/oxide-app/src/panels/footprint_editor_properties/snap_options.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/snap_options/render_snapping_mode_row
language: rust
---

# render_snapping_mode_row

v0.18.14.3 — Altium "Snapping" 3-segment toggle. `All Layers` is

## Signature

```rust
pub(super) fn render_snapping_mode_row(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    primary: Color,
    muted: Color,
    _border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.18.14.3 — Altium "Snapping" 3-segment toggle. `All Layers` is
the default behaviour (current pre-v0.18.14 functionality);
`Current Layer` is a placeholder for the v0.18.15 layer-aware
enforcement; `Off` short-circuits every snap priority in
`snap::snap_cursor` so the cursor returns the raw click.

## Source
Lines 14–81 in `crates/oxide-app/src/panels/footprint_editor_properties/snap_options.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap_options](/crates/oxide-app/src/panels/footprint_editor_properties/snap_options.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
