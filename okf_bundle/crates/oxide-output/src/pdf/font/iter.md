---
okf_version: "0.2"
type: Function
title: iter
description: Iterate over all registered fonts.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/iter
language: rust
---

# iter

Iterate over all registered fonts.

## Signature

```rust
impl FontCatalog { pub fn iter(&self) -> impl Iterator<Item = (PdfFont, Ref)> + '_ }
```

## Visibility

- `pub`

## Docstring

Iterate over all registered fonts.

## Source
Lines 260–262 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
