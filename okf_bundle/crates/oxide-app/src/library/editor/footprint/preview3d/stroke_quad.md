---
okf_version: "0.2"
type: Function
title: stroke_quad
resource: crates/oxide-app/src/library/editor/footprint/preview3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/preview3d/stroke_quad
language: rust
---

# stroke_quad

## Signature

```rust
fn stroke_quad(frame: &mut canvas::Frame, pts: &[Point; 4], color: Color)
```

## Source
Lines 254–263 in `crates/oxide-app/src/library/editor/footprint/preview3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview3d](/crates/oxide-app/src/library/editor/footprint/preview3d.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| called_by | [draw](/crates/oxide-app/src/library/editor/footprint/preview3d/draw.md) |
