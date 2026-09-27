---
okf_version: "0.2"
type: Function
title: for_style
description: Map template FontStyle to the appropriate font.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/for_style
language: rust
---

# for_style

Map template FontStyle to the appropriate font.

## Signature

```rust
impl PdfFont { pub fn for_style(style: FontStyle) -> Self }
```

## Visibility

- `pub`

## Docstring

Map template FontStyle to the appropriate font.
Normal → Roboto, Bold → Roboto Bold, Italic/BoldItalic → Iosevka.

## Source
Lines 51–58 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
