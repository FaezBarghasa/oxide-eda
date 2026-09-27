---
okf_version: "0.2"
type: Function
title: best_alias_for_text
description: Choose the most suitable alias for the given text.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/best_alias_for_text
language: rust
---

# best_alias_for_text

Choose the most suitable alias for the given text.

## Signature

```rust
pub fn best_alias_for_text(preferred_alias: &str, text: &str) -> &'static str
```

## Visibility

- `pub`

## Docstring

Choose the most suitable alias for the given text.
Keeps the preferred alias when glyph coverage is full.

## Source
Lines 124–141 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
| calls | [font_for_alias](/crates/oxide-output/src/pdf/font/font_for_alias.md) |
| calls | [glyph_coverage](/crates/oxide-output/src/pdf/font/glyph_coverage.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
