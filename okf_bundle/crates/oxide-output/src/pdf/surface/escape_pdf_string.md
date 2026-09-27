---
okf_version: "0.2"
type: Function
title: escape_pdf_string
description: Escape a string for use in PDF string literals (minimal escaping).
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/escape_pdf_string
language: rust
---

# escape_pdf_string

Escape a string for use in PDF string literals (minimal escaping).

## Signature

```rust
fn escape_pdf_string(s: &str) -> String
```

## Docstring

Escape a string for use in PDF string literals (minimal escaping).

MD-28: PDF string literals are PDFDocEncoding / Latin-1; multi-byte
UTF-8 sequences emitted raw would be misinterpreted by readers as
pairs of Latin-1 characters. Callers must run `sanitize_pdf_text`
first so the input is ASCII-only by the time it reaches us; this
`debug_assert!` enforces that contract in dev builds.

## Source
Lines 174–193 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
| called_by | [text_at](/crates/oxide-output/src/pdf/surface/text_at.md) |
| called_by | [text_at_rotated](/crates/oxide-output/src/pdf/surface/text_at_rotated.md) |
