---
okf_version: "0.2"
type: Function
title: font_data
description: Get all embedded font bytes for later embedding. Maps font to its TTF bytes.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/font_data_1
language: rust
---

# font_data

Get all embedded font bytes for later embedding. Maps font to its TTF bytes.

## Signature

```rust
pub fn font_data(&self) -> Vec<(PdfFont, &'static [u8])>
```

## Visibility

- `pub`

## Docstring

Get all embedded font bytes for later embedding. Maps font to its TTF bytes.

## Source
Lines 265–270 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
