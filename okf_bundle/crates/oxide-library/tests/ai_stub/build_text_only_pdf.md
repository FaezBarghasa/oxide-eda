---
okf_version: "0.2"
type: Function
title: build_text_only_pdf
description: "Emit a single-page PDF whose content stream draws each `line` on a"
resource: crates/oxide-library/tests/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/ai_stub/build_text_only_pdf
language: rust
---

# build_text_only_pdf

Emit a single-page PDF whose content stream draws each `line` on a

## Signature

```rust
fn build_text_only_pdf(lines: &[&str]) -> Vec<u8>
```

## Docstring

Emit a single-page PDF whose content stream draws each `line` on a
successive baseline using the standard-14 Helvetica font.

## Source
Lines 68–113 in `crates/oxide-library/tests/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/tests/ai_stub.md) |
| calls | [show](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/show.md) |
| called_by | [generate_garbage_pdf](/crates/oxide-library/tests/ai_stub/generate_garbage_pdf.md) |
| called_by | [generate_lm317_pdf](/crates/oxide-library/tests/ai_stub/generate_lm317_pdf.md) |
