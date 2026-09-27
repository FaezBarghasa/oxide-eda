---
okf_version: "0.2"
type: Function
title: get
description: "Get the Ref for a registered font, or None."
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/get
language: rust
---

# get

Get the Ref for a registered font, or None.

## Signature

```rust
impl FontCatalog { pub fn get(&self, font: PdfFont) -> Option<Ref> }
```

## Visibility

- `pub`

## Docstring

Get the Ref for a registered font, or None.

## Source
Lines 255–257 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
