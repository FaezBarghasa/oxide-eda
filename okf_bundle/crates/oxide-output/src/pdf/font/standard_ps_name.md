---
okf_version: "0.2"
type: Function
title: standard_ps_name
description: PDF standard-14 Type1 font name we fall back to while full Type0
resource: crates/oxide-output/src/pdf/font.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/pdf/font/standard_ps_name
language: rust
---

# standard_ps_name

PDF standard-14 Type1 font name we fall back to while full Type0

## Signature

```rust
impl PdfFont { pub fn standard_ps_name(&self) -> &'static str }
```

## Visibility

- `pub`

## Docstring

PDF standard-14 Type1 font name we fall back to while full Type0
composite-font emission is deferred to v0.9. Roboto → Helvetica,
Iosevka → Courier (monospace). Every PDF reader ships these by
spec, so text always renders even without a /FontFile2 stream.

## Source
Lines 75–82 in `crates/oxide-output/src/pdf/font.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [font](/crates/oxide-output/src/pdf/font.md) |
