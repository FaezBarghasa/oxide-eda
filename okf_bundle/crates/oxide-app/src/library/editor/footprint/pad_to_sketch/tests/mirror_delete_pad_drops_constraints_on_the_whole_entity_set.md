---
okf_version: "0.2"
type: Function
title: mirror_delete_pad_drops_constraints_on_the_whole_entity_set
description: "The delete sweep drops the pad's whole entity set, so a constraint"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_constraints_on_the_whole_entity_set
language: rust
---

# mirror_delete_pad_drops_constraints_on_the_whole_entity_set

The delete sweep drops the pad's whole entity set, so a constraint

## Signature

```rust
fn mirror_delete_pad_drops_constraints_on_the_whole_entity_set()
```

## Decorators

- `test`

## Docstring

The delete sweep drops the pad's whole entity set, so a constraint
authored against ANY of those entities is dangling afterwards — not
just one authored against the centre. Matching the centre id alone
left the rest behind pointing at entities that no longer exist.

It matters more now that a frame transform re-mints through this
path: a user who constrains a chamfer anchor accumulates a stale
row on every rotate and flip, not once on a pad delete.
[test]

## Source
Lines 824–873 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
