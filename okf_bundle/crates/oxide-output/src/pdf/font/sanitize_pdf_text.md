---
okf_version: "0.2"
type: Function
title: sanitize_pdf_text
description: Convert text to a PDF-safe Latin-1-ish representation so standard-14
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/sanitize_pdf_text
language: rust
---

# sanitize_pdf_text

Convert text to a PDF-safe Latin-1-ish representation so standard-14

## Signature

```rust
pub fn sanitize_pdf_text(text: &str) -> String
```

## Visibility

- `pub`

## Docstring

Convert text to a PDF-safe Latin-1-ish representation so standard-14
font fallback does not drop characters.

## Source
Lines 145–174 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
