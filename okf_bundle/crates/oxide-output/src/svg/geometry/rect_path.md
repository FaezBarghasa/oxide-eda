---
okf_version: "0.2"
type: Function
title: rect_path
resource: crates/oxide-output/src/svg/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/geometry/rect_path
language: rust
---

# rect_path

## Signature

```rust
pub(super) fn rect_path(x: f32, y: f32, w: f32, h: f32, style: SvgStyle) -> SvgElement
```

## Visibility

- `pub(super)`

## Source
Lines 26–37 in `crates/oxide-output/src/svg/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-output/src/svg/geometry.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
| called_by | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
| called_by | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
