---
okf_version: "0.2"
type: Function
title: label_size_pt
resource: crates/oxide-output/src/svg/labels.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/labels/label_size_pt
language: rust
---

# label_size_pt

## Signature

```rust
pub(super) fn label_size_pt(font_size_mm: f64, mm_to_unit: f64, scale: &PdfScale) -> f32
```

## Visibility

- `pub(super)`

## Source
Lines 16–24 in `crates/oxide-output/src/svg/labels.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [labels](/crates/oxide-output/src/svg/labels.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
