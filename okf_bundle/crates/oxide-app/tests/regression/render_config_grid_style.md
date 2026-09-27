---
okf_version: "0.2"
type: Module
title: render_config_grid_style
description: "#630 — the visible-grid style must reach the canvases as app state,"
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style
language: rust
---

# render_config_grid_style

#630 — the visible-grid style must reach the canvases as app state,

## Docstring

#630 — the visible-grid style must reach the canvases as app state,
not through a process global.

`render_config` used to keep a `OnceLock<RwLock<CanvasTextConfig>>`
that the schematic and symbol `draw` paths read directly. One global
write reached every window for free, which is exactly what made the
duplication invisible: nothing here could have failed, because there
was nothing per-canvas to get out of step. Now the value is a field on
each `CanvasSlot` plus `PanelContext::symbol_grid_style`, so the
sweep is code that can be wrong — these tests are what stops a second
window (or a Discard) from silently rendering a stale glyph.

**No test here touches the filesystem** — same reason as
`preferences_symbol_drafts.rs`: `PrefMsg::Save` writes the per-process
prefs path guarded by another module's `Mutex`, and the regression
tests are one binary with no lock shared across modules.

## Relationships

| Type | Target |
|------|--------|
| related | [inner](/crates/oxide-app/tests/regression/render_config_grid_style/inner.md) |
| related | [other_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style/other_grid_style.md) |
| related | [boot_seeds_the_grid_style_render_input_from_the_saved_pref](/crates/oxide-app/tests/regression/render_config_grid_style/boot_seeds_the_grid_style_render_input_from_the_saved_pref.md) |
| related | [drafting_the_grid_style_previews_on_the_canvas_without_committing](/crates/oxide-app/tests/regression/render_config_grid_style/drafting_the_grid_style_previews_on_the_canvas_without_committing.md) |
| related | [discarding_puts_the_canvas_grid_style_back](/crates/oxide-app/tests/regression/render_config_grid_style/discarding_puts_the_canvas_grid_style_back.md) |
| related | [no_per_window_copy_of_the_grid_style_exists](/crates/oxide-app/tests/regression/render_config_grid_style/no_per_window_copy_of_the_grid_style_exists.md) |
| related | [a_panel_ctx_rebuild_keeps_the_symbol_grid_preview](/crates/oxide-app/tests/regression/render_config_grid_style/a_panel_ctx_rebuild_keeps_the_symbol_grid_preview.md) |
