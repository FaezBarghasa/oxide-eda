---
okf_version: "0.2"
type: Function
title: resizing_a_chamfered_pad_regenerates_its_chamfer_anchor
description: "THE INVARIANT (e), the size / shape funnel. `with_selected_pad`"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/resizing_a_chamfered_pad_regenerates_its_chamfer_anchor
language: rust
---

# resizing_a_chamfered_pad_regenerates_its_chamfer_anchor

THE INVARIANT (e), the size / shape funnel. `with_selected_pad`

## Signature

```rust
fn resizing_a_chamfered_pad_regenerates_its_chamfer_anchor()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (e), the size / shape funnel. `with_selected_pad`
carries `size_mm` and `shape` edits, both of which move the whole
outline. Widening a Chamfered pad through the bbox-corner mover
pushed the corners out to the new extents and left the chamfer
anchors on the old ones.
[test]

## Source
Lines 408–435 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
