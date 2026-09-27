---
okf_version: "0.2"
type: Function
title: issue142_reopened_pad_delete_removes_its_geometry
description: "Deleting a reopened pad must remove it, not leave a ghost."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_delete_removes_its_geometry
language: rust
---

# issue142_reopened_pad_delete_removes_its_geometry

Deleting a reopened pad must remove it, not leave a ghost.

## Signature

```rust
fn issue142_reopened_pad_delete_removes_its_geometry()
```

## Decorators

- `test`

## Docstring

Deleting a reopened pad must remove it, not leave a ghost.

Same missing link as above, other mirror: `mirror_delete_pad_from_sketch`
early-returned, so the outline AND its `PadAttr`-carrying centre
stayed in the sketch. The sketch is the bake's source of truth, so
the "deleted" pad came straight back on the next bake.
[test]

## Source
Lines 145–160 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
