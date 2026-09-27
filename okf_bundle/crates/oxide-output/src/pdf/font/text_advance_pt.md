---
okf_version: "0.2"
type: Function
title: text_advance_pt
description: Approximate text advance using embedded TTF metrics at the given size.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/text_advance_pt
language: rust
---

# text_advance_pt

Approximate text advance using embedded TTF metrics at the given size.

## Signature

```rust
pub fn text_advance_pt(alias: &str, text: &str, size_pt: f32) -> f32
```

## Visibility

- `pub`

## Docstring

Approximate text advance using embedded TTF metrics at the given size.

## Source
Lines 177–201 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
| calls | [font_for_alias](/crates/oxide-output/src/pdf/font/font_for_alias.md) |
| called_by | [build_page_content](/crates/oxide-output/src/pdf/content/build_page_content.md) |
