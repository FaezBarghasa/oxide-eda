---
okf_version: "0.2"
type: Function
title: v026e_paste_does_not_alias_template_shape_params
description: "A pasted pad must not alias the template's sketch parameters."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/v026e_paste_does_not_alias_template_shape_params
language: rust
---

# v026e_paste_does_not_alias_template_shape_params

A pasted pad must not alias the template's sketch parameters.

## Signature

```rust
fn v026e_paste_does_not_alias_template_shape_params()
```

## Decorators

- `test`

## Docstring

A pasted pad must not alias the template's sketch parameters.

`shape_params` is the third pad-ownership field (alongside
`sketch_entity_id` and `corner_entity_ids`) and was cloned wholesale
from the template while only the other two were reset. The pasted
pad therefore still named the TEMPLATE's parameter names and Arc
ids, so editing its corner radius in the Properties panel resolved
through `pad.shape_params[key]` and silently resized the ORIGINAL
pad. It never self-corrected either: `auto_mint_for_literal_pads`
early-returns once the sketch holds any non-construction entity, and
`refresh_pads_from_primitive` re-attaches pads by number from the
old links, preserving the stale ledger indefinitely.
[test]

## Source
Lines 427–477 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [app_with_footprint_pads](/crates/oxide-app/tests/footprint_pad_sketch_mirror/app_with_footprint_pads.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
