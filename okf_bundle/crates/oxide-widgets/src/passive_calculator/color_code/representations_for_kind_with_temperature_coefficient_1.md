---
okf_version: "0.2"
type: Function
title: representations_for_kind_with_temperature_coefficient
resource: crates/oxide-widgets/src/passive_calculator/color_code.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/color_code/representations_for_kind_with_temperature_coefficient_1
language: rust
---

# representations_for_kind_with_temperature_coefficient

## Signature

```rust
pub fn representations_for_kind_with_temperature_coefficient(
        kind: ComponentKind,
        component: PreferredComponent,
        tolerance: Tolerance,
        temperature_coefficient: Option<TemperatureCoefficient>,
    ) -> Vec<Self>
```

## Visibility

- `pub`

## Source
Lines 170–246 in `crates/oxide-widgets/src/passive_calculator/color_code.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [color_code](/crates/oxide-widgets/src/passive_calculator/color_code.md) |
| calls | [capacitor_tolerance_color](/crates/oxide-widgets/src/passive_calculator/color_code/capacitor_tolerance_color.md) |
| calls | [adjusted_significand](/crates/oxide-widgets/src/passive_calculator/color_code/adjusted_significand.md) |
