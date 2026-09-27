---
okf_version: "0.2"
type: Function
title: rotate_delta_to_local_mm
description: "Inverse of [`Self::rotate_delta_to_world_mm`]."
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_local_mm
language: rust
---

# rotate_delta_to_local_mm

Inverse of [`Self::rotate_delta_to_world_mm`].

## Signature

```rust
impl EditorPad { pub fn rotate_delta_to_local_mm(&self, dx: f64, dy: f64) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Inverse of [`Self::rotate_delta_to_world_mm`].

## Source
Lines 209–215 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
