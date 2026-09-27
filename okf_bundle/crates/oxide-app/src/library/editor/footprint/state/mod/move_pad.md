---
okf_version: "0.2"
type: Function
title: move_pad
description: "Move the pad at `idx` to a new world position."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/move_pad
language: rust
---

# move_pad

Move the pad at `idx` to a new world position.

## Signature

```rust
impl FootprintEditorState { pub fn move_pad(&mut self, idx: usize, x_mm: f64, y_mm: f64) }
```

## Visibility

- `pub`

## Docstring

Move the pad at `idx` to a new world position.

## Source
Lines 451–456 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
