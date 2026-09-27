---
okf_version: "0.2"
type: Function
title: anchor_for_tool
description: "Look up the active tool's anchor — the previously-placed Point"
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/anchor_for_tool
language: rust
---

# anchor_for_tool

Look up the active tool's anchor — the previously-placed Point

## Signature

```rust
pub fn anchor_for_tool(
    state: &FootprintEditorState,
    sketch: Option<&SketchData>,
) -> Option<(f64, f64)>
```

## Visibility

- `pub`

## Docstring

Look up the active tool's anchor — the previously-placed Point
the next click attaches to. Returns `None` for tool states with
no anchor (Idle, Select, Place Point single-shot).

## Source
Lines 101–132 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
| calls | [point_pos](/crates/oxide-app/src/library/editor/footprint/snap/point_pos.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
