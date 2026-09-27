---
okf_version: "0.2"
type: Function
title: normalize_standard_text
resource: crates/oxide-output/src/pdf/content.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/pdf/content/normalize_standard_text
language: rust
---

# normalize_standard_text

## Signature

```rust
fn normalize_standard_text(input: &str) -> String
```

## Source
Lines 342–348 in `crates/oxide-output/src/pdf/content.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [content](/crates/oxide-output/src/pdf/content.md) |
| calls | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
| called_by | [pdf_markup_runs](/crates/oxide-output/src/pdf/content/pdf_markup_runs.md) |
