---
okf_version: "0.2"
type: Function
title: resize_round_pad
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/resize_round_pad
language: rust
---

# resize_round_pad

## Signature

```rust
fn resize_round_pad(
    editor: &mut crate::app::FootprintEditorState,
    pad_idx: usize,
    diameter_mm: f64,
)
```

## Source
Lines 519–576 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/apply.md) |
