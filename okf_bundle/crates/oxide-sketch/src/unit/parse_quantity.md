---
okf_version: "0.2"
type: Function
title: parse_quantity
description: "Parse a string like `\"0.5mm\"`, `\"100 mil\"`, `\"90deg\"`, or `\"16\"`."
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/parse_quantity
language: rust
---

# parse_quantity

Parse a string like `"0.5mm"`, `"100 mil"`, `"90deg"`, or `"16"`.

## Signature

```rust
pub fn parse_quantity(s: &str) -> Result<Quantity, UnitError>
```

## Visibility

- `pub`

## Docstring

Parse a string like `"0.5mm"`, `"100 mil"`, `"90deg"`, or `"16"`.

Accepts whitespace inside and around the input. A bare number with
no suffix is interpreted as [`Unit::Dimensionless`]. Unknown
suffixes return [`UnitError::Parse`].

## Source
Lines 165–193 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
| calls | [parse](/crates/oxide-sketch/src/expr/parse/parse.md) |
| called_by | [parse_primary](/crates/oxide-sketch/src/expr/parse/parse_primary.md) |
| called_by | [as_count_only_dimensionless](/crates/oxide-sketch/tests/unit_parser/as_count_only_dimensionless.md) |
| called_by | [as_mm_rejects_angle](/crates/oxide-sketch/tests/unit_parser/as_mm_rejects_angle.md) |
| called_by | [as_rad_rejects_length](/crates/oxide-sketch/tests/unit_parser/as_rad_rejects_length.md) |
| called_by | [longest_suffix_match](/crates/oxide-sketch/tests/unit_parser/longest_suffix_match.md) |
| called_by | [negative_and_decimal](/crates/oxide-sketch/tests/unit_parser/negative_and_decimal.md) |
| called_by | [parse_deg_to_rad](/crates/oxide-sketch/tests/unit_parser/parse_deg_to_rad.md) |
| called_by | [parse_dimensionless](/crates/oxide-sketch/tests/unit_parser/parse_dimensionless.md) |
| called_by | [parse_in_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_in_to_mm.md) |
| called_by | [parse_mil_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_mil_to_mm.md) |
| called_by | [parse_mismatch_unit_fails](/crates/oxide-sketch/tests/unit_parser/parse_mismatch_unit_fails.md) |
| called_by | [parse_mm](/crates/oxide-sketch/tests/unit_parser/parse_mm.md) |
| called_by | [parse_rad_identity](/crates/oxide-sketch/tests/unit_parser/parse_rad_identity.md) |
| called_by | [parse_um_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_um_to_mm.md) |
| called_by | [quantity_serde_round_trip](/crates/oxide-sketch/tests/unit_parser/quantity_serde_round_trip.md) |
| called_by | [whitespace_tolerated](/crates/oxide-sketch/tests/unit_parser/whitespace_tolerated.md) |
