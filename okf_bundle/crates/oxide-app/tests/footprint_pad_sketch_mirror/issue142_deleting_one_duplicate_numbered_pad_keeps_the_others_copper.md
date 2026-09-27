---
okf_version: "0.2"
type: Function
title: issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper
description: "...and deleting one of them must leave the other's copper intact."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper
language: rust
---

# issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper

...and deleting one of them must leave the other's copper intact.

## Signature

```rust
fn issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper()
```

## Decorators

- `test`

## Docstring

...and deleting one of them must leave the other's copper intact.

The concrete failure the aliasing produces. Pad A is deleted in Pads
mode; if A and B share a centre, `mirror_delete_pad_from_sketch`
takes B's outline and B's `PadAttr` centre with it while B remains
in the pad list — copper that silently vanishes from the export.
[test]

## Source
Lines 297–329 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_two_pads_sharing_a_number](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
