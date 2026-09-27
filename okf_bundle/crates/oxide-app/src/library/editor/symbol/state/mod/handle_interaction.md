---
okf_version: "0.2"
type: Function
title: handle_interaction
description: "Map a [`GraphicHandle`] to the mouse cursor that should be shown"
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/handle_interaction
language: rust
---

# handle_interaction

Map a [`GraphicHandle`] to the mouse cursor that should be shown

## Signature

```rust
pub fn handle_interaction(handle: GraphicHandle) -> mouse::Interaction
```

## Visibility

- `pub`

## Docstring

Map a [`GraphicHandle`] to the mouse cursor that should be shown
while the cursor hovers over or drags that handle.

## Source
Lines 135–159 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
| called_by | [mouse_interaction](/crates/oxide-app/src/library/editor/symbol/canvas/mod/mouse_interaction.md) |
