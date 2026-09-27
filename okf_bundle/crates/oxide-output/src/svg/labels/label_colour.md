---
okf_version: "0.2"
type: Function
title: label_colour
resource: crates/oxide-output/src/svg/labels.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/labels/label_colour
language: rust
---

# label_colour

## Signature

```rust
pub(super) fn label_colour(label_type: LabelType, palette: &SchematicPalette) -> (f32, f32, f32)
```

## Visibility

- `pub(super)`

## Source
Lines 26–33 in `crates/oxide-output/src/svg/labels.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [labels](/crates/oxide-output/src/svg/labels.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
