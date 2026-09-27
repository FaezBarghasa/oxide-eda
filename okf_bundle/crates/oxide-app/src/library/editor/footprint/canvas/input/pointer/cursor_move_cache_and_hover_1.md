---
okf_version: "0.2"
type: Function
title: cursor_move_cache_and_hover
description: Cursor-move tail — clear the cache for the tools whose ghost
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/cursor_move_cache_and_hover_1
language: rust
---

# cursor_move_cache_and_hover

Cursor-move tail — clear the cache for the tools whose ghost

## Signature

```rust
fn cursor_move_cache_and_hover(
        &self,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Cursor-move tail — clear the cache for the tools whose ghost
preview tracks the cursor, then publish the footer readout.

## Source
Lines 607–651 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
