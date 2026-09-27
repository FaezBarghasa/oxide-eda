---
okf_version: "0.2"
type: Function
title: view_footprint_library
description: "v0.18.8 — Footprint Library panel. Mirror of Altium's PCB Library"
resource: crates/oxide-app/src/panels/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/library/view_footprint_library
language: rust
---

# view_footprint_library

v0.18.8 — Footprint Library panel. Mirror of Altium's PCB Library

## Signature

```rust
pub fn view_footprint_library(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

v0.18.8 — Footprint Library panel. Mirror of Altium's PCB Library
panel: rows are the footprints *inside* the active `.snxfpt`
envelope (one per `file.footprints[i]`), with a Place / Add /
Delete / Edit button row at the bottom. Single-click highlights;
double-click (or Edit) promotes the selection to `active_idx`.

Cross-file navigation (sibling `.snxfpt` files inside the same
`.snxlib`) is reachable through the project tree — keeping the
panel single-purpose so the button row's targets are unambiguous.

## Source
Lines 311–440 in `crates/oxide-app/src/panels/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/panels/library.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [view_footprint_library_button_row](/crates/oxide-app/src/panels/library/view_footprint_library_button_row.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
