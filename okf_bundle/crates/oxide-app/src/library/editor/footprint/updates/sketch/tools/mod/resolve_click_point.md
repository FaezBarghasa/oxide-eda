---
okf_version: "0.2"
type: Function
title: resolve_click_point
description: "Resolve `effective_snap_id` into the click's entity id: an existing"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_click_point
language: rust
---

# resolve_click_point

Resolve `effective_snap_id` into the click's entity id: an existing

## Signature

```rust
fn resolve_click_point(
    editor: &mut crate::app::FootprintEditorState,
    plane_id: PlaneId,
    eff_x_mm: f64,
    eff_y_mm: f64,
    effective_snap_id: Option<SketchEntityId>,
    construction_mode: bool,
    centerline_mode: bool,
) -> SketchEntityId
```

## Docstring

Resolve `effective_snap_id` into the click's entity id: an existing
snap Point (with, for the Point tool, an Auto-Coincident constraint —
v0.22 Phase A1), or a freshly-minted Point at `(eff_x_mm, eff_y_mm)`.
Multi-click tools deliberately keep shared-ID semantics (see the
caller) — their endpoint ID is the bake's vertex identity and
switching to constraint-merged points would silently break the
closed-loop walker.

## Source
Lines 528–598 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
