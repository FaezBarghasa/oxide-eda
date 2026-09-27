---
okf_version: "0.2"
type: Function
title: owned_set_excludes_ids_with_no_live_entity
description: The owned set must never name an entity the sketch does not have.
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_set_excludes_ids_with_no_live_entity
language: rust
---

# owned_set_excludes_ids_with_no_live_entity

The owned set must never name an entity the sketch does not have.

## Signature

```rust
fn owned_set_excludes_ids_with_no_live_entity()
```

## Decorators

- `test`

## Docstring

The owned set must never name an entity the sketch does not have.

Seeds used to be pushed before the lookup that confirms them, so a
stale or foreign UUID in the ledger reached the delete drop set —
where ids are stringified and substring-matched against every
constraint's `Debug` rendering. A dead id there is not a no-op; it
deletes whatever constraint happens to mention it.
[test]

## Source
Lines 883–907 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
