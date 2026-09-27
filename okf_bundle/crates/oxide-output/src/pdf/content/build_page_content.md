---
okf_version: "0.2"
type: Function
title: build_page_content
description: Build a content stream for a single page.
resource: crates/oxide-output/src/pdf/content.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/content/build_page_content
language: rust
---

# build_page_content

Build a content stream for a single page.

## Signature

```rust
pub(super) fn build_page_content(
    sheet: &crate::SheetSnapshot,
    opts: &PdfOptions,
    ctx: &ExportContext,
    page_w_pt: f32,
    page_h_pt: f32,
    expr_tables: &ExpressionTables,
) -> Result<Vec<u8>, PdfError>
```

## Visibility

- `pub(super)`

## Docstring

Build a content stream for a single page.

## Source
Lines 6–241 in `crates/oxide-output/src/pdf/content.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [content](/crates/oxide-output/src/pdf/content.md) |
| calls | [sheet_cell_value](/crates/oxide-output/src/expression/sheet_cell_value.md) |
| calls | [pdf_markup_runs](/crates/oxide-output/src/pdf/content/pdf_markup_runs.md) |
| calls | [sanitize_pdf_text](/crates/oxide-output/src/pdf/font/sanitize_pdf_text.md) |
| calls | [best_alias_for_text](/crates/oxide-output/src/pdf/font/best_alias_for_text.md) |
| calls | [text_advance_pt](/crates/oxide-output/src/pdf/font/text_advance_pt.md) |
| calls | [rotate_about](/crates/oxide-output/src/pdf/content/rotate_about.md) |
| calls | [load_builtin](/crates/oxide-output/src/template/builtin/load_builtin.md) |
| calls | [resolve](/crates/oxide-output/src/substitution/resolve.md) |
| called_by | [export](/crates/oxide-output/src/pdf/mod/export.md) |
