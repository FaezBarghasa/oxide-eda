---
okf_version: "0.2"
type: Function
title: erc_marker_line
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/erc_marker_line
language: rust
---

# erc_marker_line

## Signature

```rust
pub(super) fn erc_marker_line(marker: &ErcMarkerInput) -> ([f32; 2], [f32; 2])
```

## Visibility

- `pub(super)`

## Source
Lines 244–255 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| called_by | [emit_erc_markers](/crates/oxide-renderer/src/schematic/emit/emit_erc_markers.md) |
