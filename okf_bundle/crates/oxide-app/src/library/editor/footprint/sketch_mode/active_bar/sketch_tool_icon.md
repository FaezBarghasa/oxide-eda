---
okf_version: "0.2"
type: Function
title: sketch_tool_icon
description: "Icon for an armed sketch tool, so a collapsed group trigger can"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/sketch_tool_icon
language: rust
---

# sketch_tool_icon

Icon for an armed sketch tool, so a collapsed group trigger can

## Signature

```rust
fn sketch_tool_icon(
    tool: SketchTool,
    theme_id: oxide_types::theme::ThemeId,
) -> iced::widget::svg::Handle
```

## Docstring

Icon for an armed sketch tool, so a collapsed group trigger can
show what's in hand instead of the generic group glyph. `Select`
and `Point` never reach a group trigger (neither belongs to one);
they fall back to the Create glyph.

## Source
Lines 373–401 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.md) |
| called_by | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
