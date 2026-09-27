---
okf_version: "0.2"
type: Class
title: AlignOp
description: v0.14 — active-bar Align / Distribute / Spacing operations. Carried
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/AlignOp
language: rust
---

# AlignOp

v0.14 — active-bar Align / Distribute / Spacing operations. Carried

## Signature

```rust
pub enum AlignOp
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.14 — active-bar Align / Distribute / Spacing operations. Carried
by [`crate::library::messages::FootprintEditorMsg::AlignPads`].
Pure data — the geometry lives in the dispatcher's `align_pads`
helper. Align variants act on ≥2 selected pads; the two Distribute
variants need ≥3.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 520–548 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
