---
okf_version: "0.2"
type: Function
title: font_for_alias
description: "Resolve alias (`F1`..`F4`) to PdfFont."
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/font_for_alias
language: rust
---

# font_for_alias

Resolve alias (`F1`..`F4`) to PdfFont.

## Signature

```rust
pub fn font_for_alias(alias: &str) -> PdfFont
```

## Visibility

- `pub`

## Docstring

Resolve alias (`F1`..`F4`) to PdfFont.

## Source
Lines 112–120 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
| called_by | [best_alias_for_text](/crates/oxide-output/src/pdf/font/best_alias_for_text.md) |
| called_by | [text_advance_pt](/crates/oxide-output/src/pdf/font/text_advance_pt.md) |
