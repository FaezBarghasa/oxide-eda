---
okf_version: "0.2"
type: Function
title: format_scaled_value
resource: crates/oxide-widgets/src/passive_calculator/rkm_code.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/rkm_code/format_scaled_value
language: rust
---

# format_scaled_value

## Signature

```rust
fn format_scaled_value(
    component: PreferredComponent,
    unit_exponent: i16,
    separator: char,
) -> String
```

## Source
Lines 215–244 in `crates/oxide-widgets/src/passive_calculator/rkm_code.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rkm_code](/crates/oxide-widgets/src/passive_calculator/rkm_code.md) |
| called_by | [value_code](/crates/oxide-widgets/src/passive_calculator/rkm_code/value_code.md) |
