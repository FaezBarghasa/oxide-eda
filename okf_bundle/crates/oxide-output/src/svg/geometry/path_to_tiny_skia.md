---
okf_version: "0.2"
type: Function
title: path_to_tiny_skia
resource: crates/oxide-output/src/svg/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/geometry/path_to_tiny_skia
language: rust
---

# path_to_tiny_skia

## Signature

```rust
pub(super) fn path_to_tiny_skia(commands: &[SvgPathCommand]) -> Option<tiny_skia::Path>
```

## Visibility

- `pub(super)`

## Source
Lines 13–24 in `crates/oxide-output/src/svg/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-output/src/svg/geometry.md) |
| called_by | [rasterize_rgba_with_colour_mode](/crates/oxide-output/src/svg/document/rasterize_rgba_with_colour_mode.md) |
