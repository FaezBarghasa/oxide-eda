---
okf_version: "0.2"
type: Function
title: mirror_delete_drops_constraints_on_owned_corners
description: "Deleting a pad must drop constraints on ANY entity it owned, not"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_constraints_on_owned_corners
language: rust
---

# mirror_delete_drops_constraints_on_owned_corners

Deleting a pad must drop constraints on ANY entity it owned, not

## Signature

```rust
fn mirror_delete_drops_constraints_on_owned_corners()
```

## Decorators

- `test`

## Docstring

Deleting a pad must drop constraints on ANY entity it owned, not
just the ones naming its centre Point.
[test]

## Source
Lines 403–428 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
