---
okf_version: "0.2"
type: Function
title: on_left_release
description: "Left release: commit a rubber-band box selection, or close the"
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_left_release
language: rust
---

# on_left_release

Left release: commit a rubber-band box selection, or close the

## Signature

```rust
impl SymbolCanvas<'_> { pub(in crate::library::editor::symbol::canvas) fn on_left_release(
        &self,
        state: &mut CanvasState,
    ) -> Option<canvas::Action<CanvasAction>> }
```

## Visibility

- `pub(in crate::library::editor::symbol::canvas)`

## Docstring

Left release: commit a rubber-band box selection, or close the
coalesced move-drag undo group.

## Source
Lines 316–358 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
| calls | [select_in_box](/crates/oxide-app/src/library/editor/symbol/state/movement/select_in_box.md) |
