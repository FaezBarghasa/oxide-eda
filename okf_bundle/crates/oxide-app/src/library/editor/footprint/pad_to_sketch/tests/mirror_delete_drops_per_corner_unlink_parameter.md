---
okf_version: "0.2"
type: Function
title: mirror_delete_drops_per_corner_unlink_parameter
description: Deleting a pad must drop the per-corner unlink override parameter.
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_per_corner_unlink_parameter
language: rust
---

# mirror_delete_drops_per_corner_unlink_parameter

Deleting a pad must drop the per-corner unlink override parameter.

## Signature

```rust
fn mirror_delete_drops_per_corner_unlink_parameter()
```

## Decorators

- `test`

## Docstring

Deleting a pad must drop the per-corner unlink override parameter.

`pad_bridge.rs` mints it as `{shared_name}_{corner_suffix}`, i.e.
`corner_r_<slug>_ne` — it ends with `_ne`, not the slug, so an
`ends_with(&slug)` retain orphaned it in the parameter table.
[test]

## Source
Lines 436–467 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
