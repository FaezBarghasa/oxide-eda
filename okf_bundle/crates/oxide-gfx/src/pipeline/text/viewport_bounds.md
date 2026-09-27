---
okf_version: "0.2"
type: Function
title: viewport_bounds
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/viewport_bounds
language: rust
---

# viewport_bounds

## Signature

```rust
fn viewport_bounds(viewport_size_px: [u32; 2]) -> glyphon::TextBounds
```

## Source
Lines 142–149 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| called_by | [clipping_helper_rejects_outside_rectangles](/crates/oxide-gfx/src/pipeline/text/clipping_helper_rejects_outside_rectangles.md) |
| called_by | [glyphon_helpers_map_size_position_and_bounds](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_size_position_and_bounds.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
