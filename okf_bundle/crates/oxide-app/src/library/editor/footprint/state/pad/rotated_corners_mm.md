---
okf_version: "0.2"
type: Function
title: rotated_corners_mm
description: "The four half-extent corners rotated about `position_mm` by"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/rotated_corners_mm
language: rust
---

# rotated_corners_mm

The four half-extent corners rotated about `position_mm` by

## Signature

```rust
impl EditorPad { pub fn rotated_corners_mm(&self) -> [(f64, f64); 4] }
```

## Visibility

- `pub`

## Docstring

The four half-extent corners rotated about `position_mm` by
`rotation_deg`, in `[ne, se, sw, nw]` order — the order the
sketch-mirror corner code already assumes.

## Source
Lines 243–248 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
