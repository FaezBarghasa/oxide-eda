---
okf_version: "0.2"
type: Function
title: rotated_offset_px
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/rotated_offset_px
language: rust
---

# rotated_offset_px

## Signature

```rust
fn rotated_offset_px(offset_px: [f32; 2], rotation_rad: f32) -> [f32; 2]
```

## Source
Lines 63–71 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| calls | [normalize_rotation_radians](/crates/oxide-gfx/src/pipeline/text/normalize_rotation_radians.md) |
| called_by | [anchored_top_left_px](/crates/oxide-gfx/src/pipeline/text/anchored_top_left_px.md) |
