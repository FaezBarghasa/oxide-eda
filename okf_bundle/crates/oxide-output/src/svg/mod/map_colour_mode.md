---
okf_version: "0.2"
type: Function
title: map_colour_mode
resource: crates/oxide-output/src/svg/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/mod/map_colour_mode
language: rust
---

# map_colour_mode

## Signature

```rust
fn map_colour_mode(rgb: (f32, f32, f32), mode: ColourMode) -> (f32, f32, f32)
```

## Source
Lines 121–138 in `crates/oxide-output/src/svg/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [svg](/crates/oxide-output/src/svg/mod.md) |
| called_by | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
