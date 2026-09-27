---
okf_version: "0.2"
type: Function
title: select_pads
resource: crates/oxide-app/src/library/editor/footprint/updates/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/selection/select_pads
language: rust
---

# select_pads

## Signature

```rust
fn select_pads(editor: &mut crate::app::FootprintEditorState, mut pads: Vec<usize>)
```

## Source
Lines 153–172 in `crates/oxide-app/src/library/editor/footprint/updates/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/updates/selection.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/selection/apply.md) |
