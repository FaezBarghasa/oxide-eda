---
okf_version: "0.2"
type: Function
title: new_npt_hole
description: v0.18.12 — non-plated through hole. No copper / mask / paste
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/new_npt_hole_1
language: rust
---

# new_npt_hole

v0.18.12 — non-plated through hole. No copper / mask / paste

## Signature

```rust
pub fn new_npt_hole(number: String, position_mm: (f64, f64), drill_mm: f64) -> Self
```

## Visibility

- `pub`

## Docstring

v0.18.12 — non-plated through hole. No copper / mask / paste
layers; the drill is the visible footprint feature.

## Source
Lines 142–171 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
