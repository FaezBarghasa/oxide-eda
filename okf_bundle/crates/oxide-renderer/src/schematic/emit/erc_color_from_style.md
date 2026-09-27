---
okf_version: "0.2"
type: Function
title: erc_color_from_style
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/erc_color_from_style
language: rust
---

# erc_color_from_style

## Signature

```rust
pub(super) fn erc_color_from_style(style: StyleRef, theme: &ResolvedTheme) -> [f32; 4]
```

## Visibility

- `pub(super)`

## Source
Lines 198–209 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| called_by | [emit_erc_markers](/crates/oxide-renderer/src/schematic/emit/emit_erc_markers.md) |
