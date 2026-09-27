---
okf_version: "0.2"
type: Function
title: properties_panel_rotation_regenerates_the_chamfer_anchor
description: "THE INVARIANT (d), the Properties-panel rotation field. Structurally"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/properties_panel_rotation_regenerates_the_chamfer_anchor
language: rust
---

# properties_panel_rotation_regenerates_the_chamfer_anchor

THE INVARIANT (d), the Properties-panel rotation field. Structurally

## Signature

```rust
fn properties_panel_rotation_regenerates_the_chamfer_anchor()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (d), the Properties-panel rotation field. Structurally
identical to the active-bar Rotate arm — same frame change, same
obligation — and it is the site the branch first regenerated
through the bbox-corner mover, so the corners turned into the 90°
frame while the chamfer anchor stayed at its 0° position and the
outline crossed itself.

The Rect-shaped sibling of this test in `footprint_pad_rotation.rs`
cannot see that: for a `Rect` the outline IS the four bbox corners
and the corner mover is correct by construction. Only a parametric
shape exposes it.
[test]

## Source
Lines 375–400 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
