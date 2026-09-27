---
okf_version: "0.2"
type: Function
title: pad_at
description: Hit-test pads in reverse z-order (last-drawn = topmost).
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/pad_at_1
language: rust
---

# pad_at

Hit-test pads in reverse z-order (last-drawn = topmost).

## Signature

```rust
pub fn pad_at(&self, x_mm: f64, y_mm: f64) -> Option<usize>
```

## Visibility

- `pub`

## Docstring

Hit-test pads in reverse z-order (last-drawn = topmost).
Skips pads on hidden layers.

## Source
Lines 491–501 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
