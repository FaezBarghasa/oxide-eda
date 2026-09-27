---
okf_version: "0.2"
type: Function
title: try_consume_repick_polar_center
description: v0.23 — RepickPolarCenter intercept. Triggered by the Pattern
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/try_consume_repick_polar_center
language: rust
---

# try_consume_repick_polar_center

v0.23 — RepickPolarCenter intercept. Triggered by the Pattern

## Signature

```rust
fn try_consume_repick_polar_center(
    editor: &mut crate::app::FootprintEditorState,
    resolved_id: SketchEntityId,
) -> bool
```

## Docstring

v0.23 — RepickPolarCenter intercept. Triggered by the Pattern
sub-form's "Re-pick centre" button. The next click on a Point
overwrites the array's `center`, independent of the active tool.
`resolved_id` is either an existing Point (when snap hit) or a
freshly-minted Point at the click location. Returns `true` when the
click was consumed by the intercept (caller must skip the per-tool
dispatch below).

## Source
Lines 607–629 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
