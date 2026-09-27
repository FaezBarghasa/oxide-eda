---
okf_version: "0.2"
type: Class
title: PolygonVertex
description: "[repr(C)]"
resource: crates/oxide-gfx/src/pipeline/polygon.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/polygon/PolygonVertex
language: rust
---

# PolygonVertex

[repr(C)]

## Signature

```rust
pub(crate) struct PolygonVertex
```

## Decorators

- `repr(C)`
- `derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)`

## Visibility

- `pub(crate)`

## Docstring

[repr(C)]
[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]

## Methods

- `position`
- `color`

## Source
Lines 12–15 in `crates/oxide-gfx/src/pipeline/polygon.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polygon](/crates/oxide-gfx/src/pipeline/polygon.md) |
| called_by | [graphic_handle_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/graphic_handle_to_msg.md) |
| called_by | [graphic_handles](/crates/oxide-app/src/library/editor/symbol/state/hit_test/graphic_handles.md) |
| called_by | [move_graphic_handle_moves_polygon_vertex](/crates/oxide-app/src/library/editor/symbol/state/tests/move_graphic_handle_moves_polygon_vertex.md) |
| called_by | [graphic_handle_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/graphic_handle_msg_to_state.md) |
