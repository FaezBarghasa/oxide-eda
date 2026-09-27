---
okf_version: "0.2"
type: Function
title: emit_overlays
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/emit_overlays
language: rust
---

# emit_overlays

## Signature

```rust
pub(super) fn emit_overlays(snapshot: &SchematicSnapshot, scene: &mut Scene)
```

## Visibility

- `pub(super)`

## Source
Lines 164–186 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| calls | [emit_overlay_line_bucket](/crates/oxide-renderer/src/schematic/emit/emit_overlay_line_bucket.md) |
| calls | [emit_overlay_polygon_bucket](/crates/oxide-renderer/src/schematic/emit/emit_overlay_polygon_bucket.md) |
| calls | [emit_overlay_circle_bucket](/crates/oxide-renderer/src/schematic/emit/emit_overlay_circle_bucket.md) |
