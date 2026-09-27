---
okf_version: "0.2"
type: Module
title: render_config
description: "Render-configuration *types* for oxide-app — the enums the appearance"
resource: crates/oxide-app/src/render_config.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/render_config
language: rust
---

# render_config

Render-configuration *types* for oxide-app — the enums the appearance

## Docstring

Render-configuration *types* for oxide-app — the enums the appearance
preferences are expressed in, plus the two shared render helpers.

#630 — this module used to also own a process-wide
`OnceLock<RwLock<CanvasTextConfig>>` that duplicated nine `UiState`
fields, was kept in sync by hand across 26 call sites, and leaked a
fresh `Box::leak`ed font name on every font change. It is gone: the
values live in `UiState` alone and reach the canvases as ordinary
fields (`CanvasSlot::grid_style`, `SymbolCanvas::grid_style` via
`PanelContext::symbol_grid_style`). Nothing here holds state — do not
reintroduce a global; a `draw` path that reads one is not a function
of the app state.

## Relationships

| Type | Target |
|------|--------|
| related | [PowerPortStyle](/crates/oxide-app/src/render_config/PowerPortStyle.md) |
| related | [LabelStyle](/crates/oxide-app/src/render_config/LabelStyle.md) |
| related | [MultisheetStyle](/crates/oxide-app/src/render_config/MultisheetStyle.md) |
| related | [GridStyle](/crates/oxide-app/src/render_config/GridStyle.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [PinSelectionMode](/crates/oxide-app/src/render_config/PinSelectionMode.md) |
| related | [allows_label_grab](/crates/oxide-app/src/render_config/allows_label_grab.md) |
| related | [pref_token](/crates/oxide-app/src/render_config/pref_token.md) |
| related | [from_pref_token](/crates/oxide-app/src/render_config/from_pref_token.md) |
| related | [allows_label_grab](/crates/oxide-app/src/render_config/allows_label_grab.md) |
| related | [pref_token](/crates/oxide-app/src/render_config/pref_token.md) |
| related | [from_pref_token](/crates/oxide-app/src/render_config/from_pref_token.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [fmt](/crates/oxide-app/src/render_config/fmt.md) |
| related | [to_iced](/crates/oxide-app/src/render_config/to_iced.md) |
