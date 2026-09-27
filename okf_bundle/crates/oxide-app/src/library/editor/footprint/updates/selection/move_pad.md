---
okf_version: "0.2"
type: Function
title: move_pad
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/move_pad
language: rust
---

# move_pad

## Signature

```rust
fn move_pad(editor: &mut crate::app::FootprintEditorState, idx: usize, x_mm: f64, y_mm: f64)
```

## Source
Lines 112–123 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
