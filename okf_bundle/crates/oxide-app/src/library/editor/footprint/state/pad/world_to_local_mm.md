---
okf_version: "0.2"
type: Function
title: world_to_local_mm
description: "Inverse of [`Self::local_to_world_mm`] — takes a world point into"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/world_to_local_mm
language: rust
---

# world_to_local_mm

Inverse of [`Self::local_to_world_mm`] — takes a world point into

## Signature

```rust
impl EditorPad { pub fn world_to_local_mm(&self, x: f64, y: f64) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Inverse of [`Self::local_to_world_mm`] — takes a world point into
the pad's own frame, where the axis-aligned reasoning that
`bbox_mm` supports is valid again.

## Source
Lines 234–238 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
