---
okf_version: "0.2"
type: Function
title: point_pos
description: "World-mm position of a sketch `Point`. Prefers the solver's last"
resource: crates/oxide-app/src/library/editor/footprint/snap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/snap/point_pos
language: rust
---

# point_pos

World-mm position of a sketch `Point`. Prefers the solver's last

## Signature

```rust
pub fn point_pos(
    id: SketchEntityId,
    sketch: Option<&SketchData>,
    state: &FootprintEditorState,
) -> Option<(f64, f64)>
```

## Visibility

- `pub`

## Docstring

World-mm position of a sketch `Point`. Prefers the solver's last
solved coordinates, falls back to authored coords. Returns `None`
if the entity isn't a `Point` or the sketch is missing.

## Source
Lines 137–161 in `crates/oxide-app/src/library/editor/footprint/snap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [snap](/crates/oxide-app/src/library/editor/footprint/snap.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [anchor_for_tool](/crates/oxide-app/src/library/editor/footprint/snap/anchor_for_tool.md) |
| called_by | [snap_cursor](/crates/oxide-app/src/library/editor/footprint/snap/snap_cursor.md) |
