---
okf_version: "0.2"
type: Function
title: shapes_dropdown_has_exactly_one_arc_row
description: "#373: the footprint Shapes dropdown once offered three rows — \"Arc"
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_dropdown_has_exactly_one_arc_row
language: rust
---

# shapes_dropdown_has_exactly_one_arc_row

#373: the footprint Shapes dropdown once offered three rows — "Arc

## Signature

```rust
fn shapes_dropdown_has_exactly_one_arc_row()
```

## Decorators

- `test`

## Docstring

#373: the footprint Shapes dropdown once offered three rows — "Arc
(Center)", "Arc (Edge)", "Arc (Any Angle)" — all arming the single
`SketchTool::Arc` gesture, silently handing the user a gesture they did
not pick. The fix collapses them to one "Arc" row. Guard against a
duplicate arc row returning. ("Fill" / "Solid Region" are a deliberate
synonym pair for `PadsTool::PlaceRegion`; this SketchTool-scoped arc
check does not touch them.)
[test]

## Source
Lines 824–842 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| calls | [shapes_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/shapes_entries.md) |
