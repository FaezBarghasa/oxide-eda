---
okf_version: "0.2"
type: Function
title: pdf_markup_runs
resource: crates/oxide-output/src/pdf/content.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/content/pdf_markup_runs
language: rust
---

# pdf_markup_runs

## Signature

```rust
fn pdf_markup_runs(input: &str) -> Vec<PdfTextRun>
```

## Source
Lines 283–340 in `crates/oxide-output/src/pdf/content.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [content](/crates/oxide-output/src/pdf/content.md) |
| calls | [normalize_standard_text](/crates/oxide-output/src/pdf/content/normalize_standard_text.md) |
| calls | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
