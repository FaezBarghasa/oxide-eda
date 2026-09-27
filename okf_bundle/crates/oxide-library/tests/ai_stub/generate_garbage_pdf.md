---
okf_version: "0.2"
type: Function
title: generate_garbage_pdf
description: "Build a minimal \"garbage\" PDF — text content with no pin-table rows."
resource: crates/oxide-library/tests/ai_stub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/ai_stub/generate_garbage_pdf
language: rust
---

# generate_garbage_pdf

Build a minimal "garbage" PDF — text content with no pin-table rows.

## Signature

```rust
fn generate_garbage_pdf() -> Vec<u8>
```

## Docstring

Build a minimal "garbage" PDF — text content with no pin-table rows.

## Source
Lines 57–64 in `crates/oxide-library/tests/ai_stub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ai_stub](/crates/oxide-library/tests/ai_stub.md) |
| calls | [build_text_only_pdf](/crates/oxide-library/tests/ai_stub/build_text_only_pdf.md) |
| called_by | [ensure_fixtures](/crates/oxide-library/tests/ai_stub/ensure_fixtures.md) |
