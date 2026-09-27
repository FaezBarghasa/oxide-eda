---
okf_version: "0.2"
type: Function
title: select_active_idx
description: v0.18.7 — switch the active footprint within the multi-
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/select_active_idx
language: rust
---

# select_active_idx

v0.18.7 — switch the active footprint within the multi-

## Signature

```rust
fn select_active_idx(editor: &mut crate::app::FootprintEditorState, idx: usize)
```

## Docstring

v0.18.7 — switch the active footprint within the multi-
footprint envelope. Resets the canvas pad list off the
newly-active primitive, clears selection, refits the
camera on the next frame so a different-sized footprint
doesn't open at a stale zoom.

## Source
Lines 49–63 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
