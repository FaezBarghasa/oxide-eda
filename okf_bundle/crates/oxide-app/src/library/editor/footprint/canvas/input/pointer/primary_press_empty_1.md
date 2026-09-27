---
okf_version: "0.2"
type: Function
title: primary_press_empty
description: Empty-area press — stash a pending click-add drag (commit on
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/pointer/primary_press_empty_1
language: rust
---

# primary_press_empty

Empty-area press — stash a pending click-add drag (commit on

## Signature

```rust
fn primary_press_empty(
        &self,
        cstate: &mut FootprintCanvasState,
        cursor_pos: Point,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Empty-area press — stash a pending click-add drag (commit on
release) and, for the Select tool, arm the rubber-band anchor.

## Source
Lines 208–237 in `crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/footprint/canvas/input/pointer.md) |
