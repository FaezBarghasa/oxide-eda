---
okf_version: "0.2"
type: Function
title: push_sch_drawing_path
resource: crates/oxide-output/src/svg/drawings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/drawings/push_sch_drawing_path
language: rust
---

# push_sch_drawing_path

## Signature

```rust
pub(super) fn push_sch_drawing_path(
    out: &mut Vec<SvgElement>,
    drawing: &SchDrawing,
    xform: &PageTransform,
    palette: &SchematicPalette,
)
```

## Visibility

- `pub(super)`

## Source
Lines 15–142 in `crates/oxide-output/src/svg/drawings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawings](/crates/oxide-output/src/svg/drawings.md) |
| calls | [rect_path](/crates/oxide-output/src/svg/geometry/rect_path.md) |
| calls | [fill_to_rgb](/crates/oxide-output/src/svg/mod/fill_to_rgb.md) |
| calls | [pt](/crates/oxide-output/src/svg/mod/pt.md) |
| calls | [circle_path](/crates/oxide-output/src/svg/geometry/circle_path.md) |
| calls | [arc_path_commands](/crates/oxide-output/src/svg/geometry/arc_path_commands.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
