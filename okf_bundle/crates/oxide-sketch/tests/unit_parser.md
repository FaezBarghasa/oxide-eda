---
okf_version: "0.2"
type: Module
title: unit_parser
description: Integration tests for the strict-unit parser
resource: crates/oxide-sketch/tests/unit_parser.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/unit_parser
language: rust
---

# unit_parser

Integration tests for the strict-unit parser

## Docstring

Integration tests for the strict-unit parser
(`crates/oxide-sketch/src/unit.rs`).

Covers Task 4.1 of `docs/internal/SKETCH_MODE_v0.13_PLAN.md`.

## Relationships

| Type | Target |
|------|--------|
| related | [parse_mm](/crates/oxide-sketch/tests/unit_parser/parse_mm.md) |
| related | [parse_mil_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_mil_to_mm.md) |
| related | [parse_in_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_in_to_mm.md) |
| related | [parse_um_to_mm](/crates/oxide-sketch/tests/unit_parser/parse_um_to_mm.md) |
| related | [parse_deg_to_rad](/crates/oxide-sketch/tests/unit_parser/parse_deg_to_rad.md) |
| related | [parse_rad_identity](/crates/oxide-sketch/tests/unit_parser/parse_rad_identity.md) |
| related | [parse_dimensionless](/crates/oxide-sketch/tests/unit_parser/parse_dimensionless.md) |
| related | [parse_mismatch_unit_fails](/crates/oxide-sketch/tests/unit_parser/parse_mismatch_unit_fails.md) |
| related | [as_mm_rejects_angle](/crates/oxide-sketch/tests/unit_parser/as_mm_rejects_angle.md) |
| related | [as_rad_rejects_length](/crates/oxide-sketch/tests/unit_parser/as_rad_rejects_length.md) |
| related | [longest_suffix_match](/crates/oxide-sketch/tests/unit_parser/longest_suffix_match.md) |
| related | [whitespace_tolerated](/crates/oxide-sketch/tests/unit_parser/whitespace_tolerated.md) |
| related | [quantity_serde_round_trip](/crates/oxide-sketch/tests/unit_parser/quantity_serde_round_trip.md) |
| related | [family_partition](/crates/oxide-sketch/tests/unit_parser/family_partition.md) |
| related | [as_count_only_dimensionless](/crates/oxide-sketch/tests/unit_parser/as_count_only_dimensionless.md) |
| related | [negative_and_decimal](/crates/oxide-sketch/tests/unit_parser/negative_and_decimal.md) |
| related | [empty_or_just_suffix_fails](/crates/oxide-sketch/tests/unit_parser/empty_or_just_suffix_fails.md) |
