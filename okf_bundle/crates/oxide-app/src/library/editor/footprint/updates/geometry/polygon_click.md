---
okf_version: "0.2"
type: Function
title: polygon_click
description: v0.18.15.4 — Place Polygon multi-click gesture. Each
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/polygon_click
language: rust
---

# polygon_click

v0.18.15.4 — Place Polygon multi-click gesture. Each

## Signature

```rust
fn polygon_click(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Docstring

v0.18.15.4 — Place Polygon multi-click gesture. Each
click appends a vertex; commit happens on tool switch /
Esc via `FootprintPolygonCommit`.

## Source
Lines 200–203 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
