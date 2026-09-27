---
okf_version: "0.2"
type: Function
title: graphic_handle_msg_to_state
description: "Translate the pure-data [`GraphicHandleMsg`] back into the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/graphic_handle_msg_to_state
language: rust
---

# graphic_handle_msg_to_state

Translate the pure-data [`GraphicHandleMsg`] back into the

## Signature

```rust
fn graphic_handle_msg_to_state(
    msg: GraphicHandleMsg,
) -> crate::library::editor::symbol::state::GraphicHandle
```

## Docstring

Translate the pure-data [`GraphicHandleMsg`] back into the
canvas-side [`crate::library::editor::symbol::state::GraphicHandle`].

## Source
Lines 574–588 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [PolygonVertex](/crates/oxide-gfx/src/pipeline/polygon/PolygonVertex.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
