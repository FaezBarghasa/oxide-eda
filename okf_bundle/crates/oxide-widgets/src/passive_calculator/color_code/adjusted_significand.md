---
okf_version: "0.2"
type: Function
title: adjusted_significand
resource: crates/oxide-widgets/src/passive_calculator/color_code.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/color_code/adjusted_significand
language: rust
---

# adjusted_significand

## Signature

```rust
fn adjusted_significand(
    mut significand: u16,
    mut multiplier_exponent: i8,
    desired_digits: usize,
) -> Option<(String, i8)>
```

## Source
Lines 266–286 in `crates/oxide-widgets/src/passive_calculator/color_code.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [color_code](/crates/oxide-widgets/src/passive_calculator/color_code.md) |
| called_by | [representations_for_kind_with_temperature_coefficient](/crates/oxide-widgets/src/passive_calculator/color_code/representations_for_kind_with_temperature_coefficient.md) |
