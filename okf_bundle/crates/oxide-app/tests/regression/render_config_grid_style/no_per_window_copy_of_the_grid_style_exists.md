---
okf_version: "0.2"
type: Function
title: no_per_window_copy_of_the_grid_style_exists
description: "#631 — an undocked window used to render from its own `CanvasSlot`"
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style/no_per_window_copy_of_the_grid_style_exists
language: rust
---

# no_per_window_copy_of_the_grid_style_exists

#631 — an undocked window used to render from its own `CanvasSlot`

## Signature

```rust
fn no_per_window_copy_of_the_grid_style_exists()
```

## Decorators

- `test`

## Docstring

#631 — an undocked window used to render from its own `CanvasSlot`
copy of the grid style, so a preview only reached it if the write swept
the whole `canvases` map. There is no copy any more: every window's
`Program` is built in `view` from the one `UiState` value, so a second
window cannot disagree with the first by construction. This test pins
that no per-window grid-style copy comes back.
[test]

## Source
Lines 120–147 in `crates/oxide-app/tests/regression/render_config_grid_style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style/other_grid_style.md) |
| calls | [inner](/crates/oxide-app/tests/regression/render_config_grid_style/inner.md) |
