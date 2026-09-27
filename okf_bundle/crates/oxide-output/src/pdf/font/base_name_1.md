---
okf_version: "0.2"
type: Function
title: base_name
description: PostScript base name for this font (used in /BaseFont). Matches the
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/base_name_1
language: rust
---

# base_name

PostScript base name for this font (used in /BaseFont). Matches the

## Signature

```rust
pub fn base_name(&self) -> &'static str
```

## Visibility

- `pub`

## Docstring

PostScript base name for this font (used in /BaseFont). Matches the
embedded TTF — used once full Type0 emission lands in v0.9.

## Source
Lines 86–93 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
