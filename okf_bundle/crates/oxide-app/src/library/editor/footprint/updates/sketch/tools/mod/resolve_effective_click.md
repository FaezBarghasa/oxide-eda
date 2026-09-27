---
okf_version: "0.2"
type: Function
title: resolve_effective_click
description: "v0.24 Track D — consume `state.placement_input` if it matches the"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_effective_click
language: rust
---

# resolve_effective_click

v0.24 Track D — consume `state.placement_input` if it matches the

## Signature

```rust
fn resolve_effective_click(
    editor: &crate::app::FootprintEditorState,
    x_mm: f64,
    y_mm: f64,
) -> (f64, f64, bool)
```

## Docstring

v0.24 Track D — consume `state.placement_input` if it matches the
active tool's pending state. The buffer is parsed as `f64` mm
(length / radius) or degrees (sweep), translated into an effective
click position overriding `x_mm` / `y_mm`. Returns the effective
`(x, y)` and a flag whose `true` value means the click was
geometry-pinned by a numeric input — used by the caller to (1) ignore
`snap_id` and (2) clear `state.placement_input` after the gesture
commits.

## Source
Lines 314–519 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
