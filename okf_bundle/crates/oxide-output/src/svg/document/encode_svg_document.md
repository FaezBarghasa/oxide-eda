---
okf_version: "0.2"
type: Function
title: encode_svg_document
resource: crates/oxide-output/src/svg/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/document/encode_svg_document
language: rust
---

# encode_svg_document

## Signature

```rust
fn encode_svg_document(width: f32, height: f32, elements: &[SvgElement]) -> String
```

## Source
Lines 461–524 in `crates/oxide-output/src/svg/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-output/src/svg/document.md) |
| calls | [to_svg_path_d](/crates/oxide-output/src/svg/document/to_svg_path_d.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
