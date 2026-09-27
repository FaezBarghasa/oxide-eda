---
okf_version: "0.2"
type: Function
title: handle_tool_click
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click
language: rust
---

# handle_tool_click

## Signature

```rust
fn handle_tool_click(
    editor: &mut crate::app::FootprintEditorState,
    x_mm: f64,
    y_mm: f64,
    snap_id: Option<SketchEntityId>,
)
```

## Source
Lines 170–283 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [resolve_sketch_plane](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_sketch_plane.md) |
| calls | [resolve_effective_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_effective_click.md) |
| calls | [resolve_click_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_click_point.md) |
| calls | [try_consume_repick_polar_center](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/try_consume_repick_polar_center.md) |
| calls | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/apply.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/apply.md) |
