---
okf_version: "0.2"
type: Function
title: label_spin_style
resource: crates/oxide-output/src/svg/labels.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/svg/labels/label_spin_style
language: rust
---

# label_spin_style

## Signature

```rust
pub(super) fn label_spin_style(justify: HAlign, rotation: f64) -> SpinStyle
```

## Visibility

- `pub(super)`

## Source
Lines 43–58 in `crates/oxide-output/src/svg/labels.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [labels](/crates/oxide-output/src/svg/labels.md) |
| calls | [normalize_rotation](/crates/oxide-output/src/svg/labels/normalize_rotation.md) |
| called_by | [from_sheet](/crates/oxide-output/src/svg/document/from_sheet.md) |
