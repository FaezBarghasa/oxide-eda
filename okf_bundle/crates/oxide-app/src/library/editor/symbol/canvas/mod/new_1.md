---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(
        symbol: &'a Symbol,
        selected: Option<SymbolSelection>,
        tool: SymbolTool,
        polygon_vertices: &'a [(f64, f64)],
        active_part: u8,
        context_menu_open: bool,
        camera: &'a crate::canvas::Camera,
        grid_size_mm: f64,
        grid_visible: bool,
        grid_style: crate::render_config::GridStyle,
        pin_label_grab: bool,
        sheet_color: Color,
        accent_color: Color,
        _body_color_unused: Color,
        _text_color_unused: Color,
        _grid_color_unused: Color,
    ) -> Self
```

## Decorators

- `expect(
        clippy::too_many_arguments,
        reason = "16 arguments: the per-frame canvas takes the symbol, theme and global grid/unit settings; see the module docs for the parity rationale"
    )`

## Visibility

- `pub`

## Source
Lines 133–178 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
