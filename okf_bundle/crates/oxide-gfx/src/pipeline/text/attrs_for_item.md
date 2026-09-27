---
okf_version: "0.2"
type: Function
title: attrs_for_item
description: "`family` is the caller's canvas font family name. It used to be a"
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/attrs_for_item
language: rust
---

# attrs_for_item

`family` is the caller's canvas font family name. It used to be a

## Signature

```rust
fn attrs_for_item(item: &TextItem, family: &'static str) -> glyphon::Attrs<'static>
```

## Docstring

`family` is the caller's canvas font family name. It used to be a
hardcoded `Family::SansSerif`, which rendered schematic text in whatever
the system served instead of the app's monospace canvas face — different
typeface *and* different advance widths, so anything sized from a glyph
estimate came out wrong too.

## Source
Lines 174–186 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| called_by | [glyphon_helpers_map_style_and_color](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_style_and_color.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
