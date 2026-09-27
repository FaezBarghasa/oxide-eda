---
okf_version: "0.2"
type: Function
title: is_sketch_profile_pad
description: "True when this pad's copper is a traced sketch loop (\"Make Pad"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/is_sketch_profile_pad
language: rust
---

# is_sketch_profile_pad

True when this pad's copper is a traced sketch loop ("Make Pad

## Signature

```rust
pub fn is_sketch_profile_pad(pad: &EditorPad, footprint: &Footprint) -> bool
```

## Visibility

- `pub`

## Docstring

True when this pad's copper is a traced sketch loop ("Make Pad
from Profile") rather than a parametric shape.

Such a pad owns no `size_mm` / `shape` geometry for a transform to
ride on — its outline is the loop. A caller that mirrors or
otherwise reshapes pad copper must either transform the loop too or
say out loud that it did not; silently leaving the loop put bakes
the un-transformed shape.

## Source
Lines 426–434 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [profile_seed_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/profile_seed_line.md) |
| called_by | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
