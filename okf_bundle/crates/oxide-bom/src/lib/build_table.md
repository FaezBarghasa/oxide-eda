---
okf_version: "0.2"
type: Function
title: build_table
description: Build a BOM table from a normalized BOM context.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/build_table
language: rust
---

# build_table

Build a BOM table from a normalized BOM context.

## Signature

```rust
pub fn build_table(ctx: &BomContext, opts: &BomEngineOptions) -> BomTable
```

## Visibility

- `pub`

## Docstring

Build a BOM table from a normalized BOM context.

## Source
Lines 188–289 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
| calls | [compare_references](/crates/oxide-types/src/designator/compare_references.md) |
| calls | [first_reference](/crates/oxide-bom/src/lib/first_reference.md) |
| called_by | [can_include_excluded_from_bom_and_not_on_board](/crates/oxide-bom/src/lib/can_include_excluded_from_bom_and_not_on_board.md) |
| called_by | [filters_dnp_by_default](/crates/oxide-bom/src/lib/filters_dnp_by_default.md) |
| called_by | [groups_by_value_and_footprint](/crates/oxide-bom/src/lib/groups_by_value_and_footprint.md) |
| called_by | [sorts_references_naturally_when_grouped](/crates/oxide-bom/src/lib/sorts_references_naturally_when_grouped.md) |
| called_by | [sorts_references_without_or_with_oversized_numeric_tails](/crates/oxide-bom/src/lib/sorts_references_without_or_with_oversized_numeric_tails.md) |
| called_by | [sorts_rows_naturally_when_flat](/crates/oxide-bom/src/lib/sorts_rows_naturally_when_flat.md) |
| called_by | [sorts_rows_naturally_when_grouped_across_rows](/crates/oxide-bom/src/lib/sorts_rows_naturally_when_grouped_across_rows.md) |
| called_by | [validation_can_disable_missing_footprint_rule](/crates/oxide-bom/src/lib/validation_can_disable_missing_footprint_rule.md) |
| called_by | [validation_reports_duplicate_designators](/crates/oxide-bom/src/lib/validation_reports_duplicate_designators.md) |
| called_by | [export](/crates/oxide-output/src/bom/mod/export.md) |
| called_by | [rollup](/crates/oxide-output/src/bom/mod/rollup.md) |
