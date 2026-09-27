---
okf_version: "0.2"
type: Function
title: label
description: Short display label for the toolbar pill — Altium nomenclature
resource: crates/oxide-app/src/library/editor/footprint/layers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/layers/label_1
language: rust
---

# label

Short display label for the toolbar pill — Altium nomenclature

## Signature

```rust
pub fn label(self) -> &'static str
```

## Visibility

- `pub`

## Docstring

Short display label for the toolbar pill — Altium nomenclature
per `docs/UX_REFERENCE_ALTIUM.md`. The Standard/data-layer name is
available via [`Self::standard_name`] for sexpr round-trips.

## Source
Lines 44–54 in `crates/oxide-app/src/library/editor/footprint/layers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layers](/crates/oxide-app/src/library/editor/footprint/layers.md) |
