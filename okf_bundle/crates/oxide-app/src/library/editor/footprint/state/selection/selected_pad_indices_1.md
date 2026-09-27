---
okf_version: "0.2"
type: Function
title: selected_pad_indices
description: Combined pad selection — primary plus ctrl-click extras —
resource: crates/oxide-app/src/library/editor/footprint/state/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection/selected_pad_indices_1
language: rust
---

# selected_pad_indices

Combined pad selection — primary plus ctrl-click extras —

## Signature

```rust
pub fn selected_pad_indices(&self) -> Vec<usize>
```

## Visibility

- `pub`

## Docstring

Combined pad selection — primary plus ctrl-click extras —
sorted, deduped, and clamped to the live pad list. Empty when
nothing is selected.

## Source
Lines 17–27 in `crates/oxide-app/src/library/editor/footprint/state/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-app/src/library/editor/footprint/state/selection.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
