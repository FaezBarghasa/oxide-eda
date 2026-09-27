---
okf_version: "0.2"
type: Function
title: arc_click
description: v0.18.15.3 — Place Arc 3-click gesture (centre / radius
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/arc_click
language: rust
---

# arc_click

v0.18.15.3 — Place Arc 3-click gesture (centre / radius

## Signature

```rust
fn arc_click(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Docstring

v0.18.15.3 — Place Arc 3-click gesture (centre / radius
start / sweep end). Idle → Center → Start → commit. After
commit the gesture resets to Idle (no chain — arcs
typically aren't strung together).

## Source
Lines 152–189 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
