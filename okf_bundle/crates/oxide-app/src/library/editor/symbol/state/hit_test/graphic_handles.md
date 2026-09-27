---
okf_version: "0.2"
type: Function
title: graphic_handles
description: "Enumerate every resize handle for the graphic at `idx`."
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/graphic_handles
language: rust
---

# graphic_handles

Enumerate every resize handle for the graphic at `idx`.

## Signature

```rust
pub fn graphic_handles(sym: &Symbol, idx: usize) -> Vec<(GraphicHandle, [f64; 2])>
```

## Visibility

- `pub`

## Docstring

Enumerate every resize handle for the graphic at `idx`.
Returns `(handle_variant, world_position)` pairs for Select-tool
handle rendering.

## Source
Lines 265–331 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [PolygonVertex](/crates/oxide-gfx/src/pipeline/polygon/PolygonVertex.md) |
| called_by | [draw_resize_handles](/crates/oxide-app/src/library/editor/symbol/canvas/draw/scene/draw_resize_handles.md) |
| called_by | [hit_test_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_handle.md) |
