---
okf_version: "0.2"
type: Function
title: generate_lm317_pdf
description: Build a small synthetic LM317-style datasheet PDF.
resource: crates/oxide-library/tests/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/ai_stub/generate_lm317_pdf
language: rust
---

# generate_lm317_pdf

Build a small synthetic LM317-style datasheet PDF.

## Signature

```rust
fn generate_lm317_pdf() -> Vec<u8>
```

## Docstring

Build a small synthetic LM317-style datasheet PDF.

The PDF embeds a textual pin table in a content stream so
`pdf_extract::extract_text_from_mem` will recover line-by-line text.

## Source
Lines 43–54 in `crates/oxide-library/tests/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/tests/ai_stub.md) |
| calls | [build_text_only_pdf](/crates/oxide-library/tests/ai_stub/build_text_only_pdf.md) |
| called_by | [ensure_fixtures](/crates/oxide-library/tests/ai_stub/ensure_fixtures.md) |
