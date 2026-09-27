---
okf_version: "0.2"
type: Function
title: font_bytes
description: Retrieve the embedded TTF bytes for this font.
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/font_bytes_1
language: rust
---

# font_bytes

Retrieve the embedded TTF bytes for this font.

## Signature

```rust
pub fn font_bytes(&self) -> &'static [u8]
```

## Visibility

- `pub`

## Docstring

Retrieve the embedded TTF bytes for this font.

## Source
Lines 96–103 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
