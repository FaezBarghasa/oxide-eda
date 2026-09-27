---
okf_version: "0.2"
type: Function
title: issue142_delete_does_not_eat_user_geometry_sharing_an_anchor
description: Deleting a pad must not delete user geometry that merely touches it.
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_delete_does_not_eat_user_geometry_sharing_an_anchor
language: rust
---

# issue142_delete_does_not_eat_user_geometry_sharing_an_anchor

Deleting a pad must not delete user geometry that merely touches it.

## Signature

```rust
fn issue142_delete_does_not_eat_user_geometry_sharing_an_anchor()
```

## Decorators

- `test`

## Docstring

Deleting a pad must not delete user geometry that merely touches it.

The delete sweep pulls in every Line / Arc that references a dropped
Point — it has to, a Line with a dead endpoint is not a Line. But it
used to also drop that entity's OTHER endpoint, which for a
user-drawn silk Line anchored to a pad corner is the far end sitting
out in the user's own drawing. Widening the pad's owned set made
that reach three times further. The pad's own anchors and inset
arc-centres are already reached by expanding its Arcs forward, so
the far side of such an edge is only ever foreign.
[test]

## Source
Lines 173–217 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
