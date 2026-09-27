---
okf_version: "0.2"
type: Function
title: schematic_text_offset_hier
resource: crates/oxide-output/src/svg/labels.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/labels/schematic_text_offset_hier
language: rust
---

# schematic_text_offset_hier

## Signature

```rust
pub(super) fn schematic_text_offset_hier(
    text: &str,
    font_size_mm: f64,
    spin: SpinStyle,
) -> (f64, f64)
```

## Visibility

- `pub(super)`

## Source
Lines 68–95 in `crates/oxide-output/src/svg/labels.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [labels](/crates/oxide-output/src/svg/labels.md) |
| calls | [parse_oxide_markup](/crates/oxide-types/src/markup/parse_oxide_markup.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
