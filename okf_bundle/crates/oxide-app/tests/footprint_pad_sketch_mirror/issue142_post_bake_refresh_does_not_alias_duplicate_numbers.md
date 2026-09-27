---
okf_version: "0.2"
type: Function
title: issue142_post_bake_refresh_does_not_alias_duplicate_numbers
description: The post-bake refresh must not alias duplicate numbers either.
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_post_bake_refresh_does_not_alias_duplicate_numbers
language: rust
---

# issue142_post_bake_refresh_does_not_alias_duplicate_numbers

The post-bake refresh must not alias duplicate numbers either.

## Signature

```rust
fn issue142_post_bake_refresh_does_not_alias_duplicate_numbers()
```

## Decorators

- `test`

## Docstring

The post-bake refresh must not alias duplicate numbers either.

`refresh_pads_from_primitive` carries the three volatile link fields
across the rebuild through its own number-keyed map — the same
last-wins structure, the same aliasing, one function over. Fixing
only the reopen path would have left every post-bake refresh
handing both duplicate-numbered pads one centre.
[test]

## Source
Lines 487–509 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_two_pads_sharing_a_number](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_two_pads_sharing_a_number.md) |
