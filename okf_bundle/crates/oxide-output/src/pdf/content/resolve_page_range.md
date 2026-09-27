---
okf_version: "0.2"
type: Function
title: resolve_page_range
description: "Resolve a `PageRange` against the project's sheet count into a concrete"
resource: crates/oxide-output/src/pdf/content.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/content/resolve_page_range
language: rust
---

# resolve_page_range

Resolve a `PageRange` against the project's sheet count into a concrete

## Signature

```rust
pub(super) fn resolve_page_range(
    range: &PageRange,
    sheet_count: usize,
) -> Result<Vec<usize>, PdfError>
```

## Visibility

- `pub(super)`

## Docstring

Resolve a `PageRange` against the project's sheet count into a concrete
list of zero-based sheet indices to export.

## Source
Lines 245–273 in `crates/oxide-output/src/pdf/content.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [content](/crates/oxide-output/src/pdf/content.md) |
| called_by | [export](/crates/oxide-output/src/pdf/mod/export.md) |
