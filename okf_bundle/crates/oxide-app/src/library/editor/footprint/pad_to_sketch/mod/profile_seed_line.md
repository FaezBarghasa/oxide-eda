---
okf_version: "0.2"
type: Function
title: profile_seed_line
description: "Seed Line of a `Custom(SketchProfile)` pad, read off the `PadAttr`"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/profile_seed_line
language: rust
---

# profile_seed_line

Seed Line of a `Custom(SketchProfile)` pad, read off the `PadAttr`

## Signature

```rust
fn profile_seed_line(sketch: &SketchData, centre: SketchEntityId) -> Option<SketchEntityId>
```

## Docstring

Seed Line of a `Custom(SketchProfile)` pad, read off the `PadAttr`
carried by its centre `Point`. `None` for every other shape — those
pads own their geometry through `corner_entity_ids` instead.

## Source
Lines 452–465 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [is_sketch_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/is_sketch_profile_pad.md) |
| called_by | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
