---
okf_version: "0.2"
type: Function
title: sketch_centre_drag_carries_the_chamfer_anchor_with_it
description: The Sketch-mode CENTRE drag — the one translation path that does not
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/sketch_centre_drag_carries_the_chamfer_anchor_with_it
language: rust
---

# sketch_centre_drag_carries_the_chamfer_anchor_with_it

The Sketch-mode CENTRE drag — the one translation path that does not

## Signature

```rust
fn sketch_centre_drag_carries_the_chamfer_anchor_with_it()
```

## Decorators

- `test`

## Docstring

The Sketch-mode CENTRE drag — the one translation path that does not
route through `mirror_move_pad_in_sketch`, so the sweep flagged it
as a candidate sibling of (f). It is NOT one, and this test is NOT a
proof of a fix: it passes on the code as it stands, and it passed
before this round too.

The handler translates the centre and the four bbox corners
explicitly, but it does so through `apply_sketch_edit_with_warnings`
— the SOLVER pass — and the chamfer anchors are constrained to the
outline, so the solve carries them. Translating them a second time
by hand, which is what the Pads-mode sites need, doubles the delta:
it lands the anchor at (10.75, 5.5) instead of (5.75, 2.5).

Kept as the guard that fact deserves. Anyone who changes this
handler to bypass the solver has to make the sidecar ride some
other way, and this is what tells them.
[test]

## Source
Lines 687–726 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
