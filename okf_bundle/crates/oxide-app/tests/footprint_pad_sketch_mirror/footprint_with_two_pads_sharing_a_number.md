---
okf_version: "0.2"
type: Function
title: footprint_with_two_pads_sharing_a_number
description: "Two pads that SHARE a pad number, both minted, both synced onto the"
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number
language: rust
---

# footprint_with_two_pads_sharing_a_number

Two pads that SHARE a pad number, both minted, both synced onto the

## Signature

```rust
fn footprint_with_two_pads_sharing_a_number() -> Footprint
```

## Docstring

Two pads that SHARE a pad number, both minted, both synced onto the
primitive — i.e. what a shared-designator row / thermal / shield pad
set looks like on disk. `designator_override` is the production path
that produces it: it stamps one number onto every pad placed after
it.

## Source
Lines 48–60 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper.md) |
| called_by | [issue142_duplicate_pad_numbers_do_not_alias_one_centre](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_duplicate_pad_numbers_do_not_alias_one_centre.md) |
| called_by | [issue142_post_bake_refresh_does_not_alias_duplicate_numbers](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_post_bake_refresh_does_not_alias_duplicate_numbers.md) |
