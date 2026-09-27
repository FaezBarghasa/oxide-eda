---
okf_version: "0.2"
type: Function
title: rotate_delta_to_world_mm
description: Rotate a free VECTOR (a delta — no translation applied) from the
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_world_mm
language: rust
---

# rotate_delta_to_world_mm

Rotate a free VECTOR (a delta — no translation applied) from the

## Signature

```rust
impl EditorPad { pub fn rotate_delta_to_world_mm(&self, dx: f64, dy: f64) -> (f64, f64) }
```

## Visibility

- `pub`

## Docstring

Rotate a free VECTOR (a delta — no translation applied) from the
pad's own frame into world mm.

## Source
Lines 200–206 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
