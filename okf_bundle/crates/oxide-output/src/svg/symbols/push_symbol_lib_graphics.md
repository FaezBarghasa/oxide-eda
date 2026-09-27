---
okf_version: "0.2"
type: Function
title: push_symbol_lib_graphics
resource: crates/oxide-output/src/svg/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics
language: rust
---

# push_symbol_lib_graphics

## Signature

```rust
pub(super) fn push_symbol_lib_graphics(
    out: &mut Vec<SvgElement>,
    sym: &Symbol,
    lib: &LibSymbol,
    xform: &PageTransform,
    palette: &SchematicPalette,
)
```

## Visibility

- `pub(super)`

## Source
Lines 20–239 in `crates/oxide-output/src/svg/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-output/src/svg/symbols.md) |
| calls | [symbol_world_point](/crates/oxide-output/src/svg/symbols/symbol_world_point.md) |
| calls | [pt](/crates/oxide-output/src/svg/mod/pt.md) |
| calls | [fill_to_rgb](/crates/oxide-output/src/svg/mod/fill_to_rgb.md) |
| calls | [circle_path](/crates/oxide-output/src/svg/geometry/circle_path.md) |
| calls | [arc_path_commands](/crates/oxide-output/src/svg/geometry/arc_path_commands.md) |
| calls | [rect_path](/crates/oxide-output/src/svg/geometry/rect_path.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
