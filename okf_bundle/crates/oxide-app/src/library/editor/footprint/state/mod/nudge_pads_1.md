---
okf_version: "0.2"
type: Function
title: nudge_pads
description: "v0.14 — translate every pad in `indices` by `(dx, dy)` mm."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/nudge_pads_1
language: rust
---

# nudge_pads

v0.14 — translate every pad in `indices` by `(dx, dy)` mm.

## Signature

```rust
pub fn nudge_pads(&mut self, indices: &[usize], dx: f64, dy: f64) -> Vec<usize>
```

## Visibility

- `pub`

## Docstring

v0.14 — translate every pad in `indices` by `(dx, dy)` mm.
Backs the active-bar "Move Selection by X, Y…" nudge. Out-of-
range indices are skipped; the courtyard is recomputed once at
the end. Returns the moved pad indices (in the order given,
minus any out-of-range entries) so the caller can mirror exactly
those pads into the backing sketch.

## Source
Lines 464–477 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
