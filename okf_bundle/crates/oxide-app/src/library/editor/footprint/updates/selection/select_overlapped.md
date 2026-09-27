---
okf_version: "0.2"
type: Function
title: select_overlapped
description: v0.27 — Cycle through pads stacked at the most recent click world
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/select_overlapped
language: rust
---

# select_overlapped

v0.27 — Cycle through pads stacked at the most recent click world

## Signature

```rust
fn select_overlapped(editor: &mut crate::app::FootprintEditorState, msg: &FootprintEditorMsg)
```

## Docstring

v0.27 — Cycle through pads stacked at the most recent click world
position. `SelectOverlapped` goes to the previous pad in z-order;
`SelectNextOverlapped` advances. Without a recorded click position
there's no stack to cycle, so the action is a silent no-op.

## Source
Lines 466–508 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
