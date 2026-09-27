---
okf_version: "0.2"
type: Function
title: update_middle_released
description: "Middle-release: stop pan."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_middle_released_1
language: rust
---

# update_middle_released

Middle-release: stop pan.

## Signature

```rust
pub(in crate::canvas) fn update_middle_released(
        &self,
        state: &mut CanvasState,
    ) -> Option<canvas::Action<Message>>
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Middle-release: stop pan.

## Source
Lines 270–277 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
