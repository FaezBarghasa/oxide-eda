---
okf_version: "0.2"
type: Function
title: a_panel_ctx_rebuild_keeps_the_symbol_grid_preview
description: "The Symbol Editor's grid style previews through `panel_ctx`, and"
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style/a_panel_ctx_rebuild_keeps_the_symbol_grid_preview
language: rust
---

# a_panel_ctx_rebuild_keeps_the_symbol_grid_preview

The Symbol Editor's grid style previews through `panel_ctx`, and

## Signature

```rust
fn a_panel_ctx_rebuild_keeps_the_symbol_grid_preview()
```

## Decorators

- `test`

## Docstring

The Symbol Editor's grid style previews through `panel_ctx`, and
`refresh_panel_ctx` rebuilds that struct from scratch on all sorts of
unrelated messages. If the rebuild re-seeded from `ui_state`, an
in-flight preview would snap back mid-dialog.
[test]

## Source
Lines 154–171 in `crates/oxide-app/tests/regression/render_config_grid_style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style.md) |
| calls | [other_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style/other_grid_style.md) |
| calls | [inner](/crates/oxide-app/tests/regression/render_config_grid_style/inner.md) |
