---
okf_version: "0.2"
type: Function
title: update_escape_pressed
description: "Escape: cancel any in-progress drag."
resource: crates/oxide-app/src/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/keys/update_escape_pressed
language: rust
---

# update_escape_pressed

Escape: cancel any in-progress drag.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn update_escape_pressed(
        &self,
        state: &mut CanvasState,
    ) -> Option<canvas::Action<Message>> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Escape: cancel any in-progress drag.

## Source
Lines 16–28 in `crates/oxide-app/src/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/canvas/input/keys.md) |
