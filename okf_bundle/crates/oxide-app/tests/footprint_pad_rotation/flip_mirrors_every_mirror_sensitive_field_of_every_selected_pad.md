---
okf_version: "0.2"
type: Function
title: flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad
description: "Flip mirrors the pad's copper to the other side. `oxide_bake::pad`"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad
language: rust
---

# flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad

Flip mirrors the pad's copper to the other side. `oxide_bake::pad`

## Signature

```rust
fn flip_mirrors_every_mirror_sensitive_field_of_every_selected_pad()
```

## Decorators

- `test`

## Docstring

Flip mirrors the pad's copper to the other side. `oxide_bake::pad`
consumes the stored fields verbatim with no side-based mirroring of
its own, so the stored data IS the geometry and the WHOLE
mirror-sensitive set has to move under `x → -x`, not just the angle.

Mirroring a subset bakes a shape that is neither the front nor the
back one: a Chamfered pad flipped with its angle negated but its
corner flags left alone keeps the chamfer on the wrong corner and
the part will not seat.
[test]

## Source
Lines 271–337 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [fixture](/crates/oxide-app/tests/footprint_pad_rotation/fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
