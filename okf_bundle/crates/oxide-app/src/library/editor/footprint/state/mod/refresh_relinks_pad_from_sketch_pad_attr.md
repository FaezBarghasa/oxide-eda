---
okf_version: "0.2"
type: Function
title: refresh_relinks_pad_from_sketch_pad_attr
description: "A pad that first appears from the SKETCH side — \"Make Pad from"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/refresh_relinks_pad_from_sketch_pad_attr
language: rust
---

# refresh_relinks_pad_from_sketch_pad_attr

A pad that first appears from the SKETCH side — "Make Pad from

## Signature

```rust
fn refresh_relinks_pad_from_sketch_pad_attr()
```

## Decorators

- `test`

## Docstring

A pad that first appears from the SKETCH side — "Make Pad from
Profile" mints a centre Point carrying a `PadAttr` directly on the
sketch, and the pad only reaches `state.pads` after the next
bake. Its number never existed in the prior `state.pads`, so the
number-vs-old-pads relink misses and it would stay permanently
unlinked (`sketch_entity_id: None`). An unlinked pad can't
mirror a move into the sketch, so dragging it in Pads mode leaves
the profile behind and the pad snaps back on the next bake.

The relink must fall back to the authoritative link: the sketch
entity whose `PadAttr.number` matches the pad.
[test]

## Source
Lines 776–814 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
