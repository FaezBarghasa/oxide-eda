---
okf_version: "0.2"
type: Function
title: other_grid_style
description: "Pick the variant the app is not currently showing, so each test asserts"
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style/other_grid_style
language: rust
---

# other_grid_style

Pick the variant the app is not currently showing, so each test asserts

## Signature

```rust
fn other_grid_style(current: GridStyle) -> GridStyle
```

## Docstring

Pick the variant the app is not currently showing, so each test asserts
on a real change rather than a no-op assignment that would also pass
against a handler that dropped the message entirely.

## Source
Lines 29–34 in `crates/oxide-app/tests/regression/render_config_grid_style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style.md) |
| called_by | [a_panel_ctx_rebuild_keeps_the_symbol_grid_preview](/crates/oxide-app/tests/regression/render_config_grid_style/a_panel_ctx_rebuild_keeps_the_symbol_grid_preview.md) |
| called_by | [discarding_puts_the_canvas_grid_style_back](/crates/oxide-app/tests/regression/render_config_grid_style/discarding_puts_the_canvas_grid_style_back.md) |
| called_by | [drafting_the_grid_style_previews_on_the_canvas_without_committing](/crates/oxide-app/tests/regression/render_config_grid_style/drafting_the_grid_style_previews_on_the_canvas_without_committing.md) |
| called_by | [no_per_window_copy_of_the_grid_style_exists](/crates/oxide-app/tests/regression/render_config_grid_style/no_per_window_copy_of_the_grid_style_exists.md) |
