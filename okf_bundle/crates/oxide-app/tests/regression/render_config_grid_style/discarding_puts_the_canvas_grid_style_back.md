---
okf_version: "0.2"
type: Function
title: discarding_puts_the_canvas_grid_style_back
description: "The regression the global hid: Discard restores the committed style on"
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style/discarding_puts_the_canvas_grid_style_back
language: rust
---

# discarding_puts_the_canvas_grid_style_back

The regression the global hid: Discard restores the committed style on

## Signature

```rust
fn discarding_puts_the_canvas_grid_style_back()
```

## Decorators

- `test`

## Docstring

The regression the global hid: Discard restores the committed style on
the canvas. With a per-canvas field, forgetting the push-back in
`revert_preferences_drafts` leaves the abandoned glyph rendering with
the picker showing the old value.
[test]

## Source
Lines 92–111 in `crates/oxide-app/tests/regression/render_config_grid_style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style.md) |
| calls | [inner](/crates/oxide-app/tests/regression/render_config_grid_style/inner.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style/other_grid_style.md) |
