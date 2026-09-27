---
okf_version: "0.2"
type: Class
title: SelectionFilter
description: Each flag gates whether the corresponding kind is selectable in
resource: crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection_filter/SelectionFilter
language: rust
---

# SelectionFilter

Each flag gates whether the corresponding kind is selectable in

## Signature

```rust
pub struct SelectionFilter
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Each flag gates whether the corresponding kind is selectable in
the canvas. `Pads` is the only one functionally wired today; the
others are stored for forward compatibility so the pill row
reflects user intent.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `pads`
- `tracks`
- `arcs`
- `pours`
- `bodies_3d`
- `keepouts`
- `cutouts`
- `texts`
- `vias`
- `regions`
- `fills`
- `other`

## Source
Lines 34–47 in `crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_filter](/crates/oxide-app/src/library/editor/footprint/state/selection_filter.md) |
