---
okf_version: "0.2"
type: Function
title: track_click
description: v0.18.15.1 — Place Track 2-click gesture. First click
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/track_click
language: rust
---

# track_click

v0.18.15.1 — Place Track 2-click gesture. First click

## Signature

```rust
fn track_click(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Docstring

v0.18.15.1 — Place Track 2-click gesture. First click
stashes the start in `state.track_first`; second click
commits the line to silk_f and chains by re-stashing the
second click as the next gesture's start.

## Source
Lines 116–141 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
