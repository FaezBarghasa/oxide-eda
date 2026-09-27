---
okf_version: "0.2"
type: Function
title: rasterize
description: Rasterise all sheets in the export context to RGBA bitmaps.
resource: crates/oxide-output/src/preview/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/preview/mod/rasterize_1
language: rust
---

# rasterize

Rasterise all sheets in the export context to RGBA bitmaps.

## Signature

```rust
pub fn rasterize(&self, ctx: &ExportContext, opts: &PreviewOptions) -> Vec<PreviewPage>
```

## Visibility

- `pub`

## Docstring

Rasterise all sheets in the export context to RGBA bitmaps.

Returns a vec of preview pages, one per sheet in sheet order.
If a page fails to rasterise (e.g., dimensions too small), it is skipped.

## Source
Lines 55–67 in `crates/oxide-output/src/preview/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-output/src/preview/mod.md) |
| calls | [build_expression_tables](/crates/oxide-output/src/expression/build_expression_tables.md) |
| calls | [resolve_page_range_preview](/crates/oxide-output/src/preview/mod/resolve_page_range_preview.md) |
| calls | [rasterize_page](/crates/oxide-output/src/preview/rasterize/rasterize_page.md) |
