---
okf_version: "0.2"
type: Function
title: validate_table
description: Run quality rules against BOM input/output and return a validation report.
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/validate_table
language: rust
---

# validate_table

Run quality rules against BOM input/output and return a validation report.

## Signature

```rust
pub fn validate_table(
    ctx: &BomContext,
    table: &BomTable,
    options: &BomRuleOptions,
) -> BomValidationReport
```

## Visibility

- `pub`

## Docstring

Run quality rules against BOM input/output and return a validation report.

## Source
Lines 301–370 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
| calls | [has_populated_field](/crates/oxide-bom/src/lib/has_populated_field.md) |
| called_by | [validation_can_disable_missing_footprint_rule](/crates/oxide-bom/src/lib/validation_can_disable_missing_footprint_rule.md) |
| called_by | [validation_reports_duplicate_designators](/crates/oxide-bom/src/lib/validation_reports_duplicate_designators.md) |
| called_by | [export](/crates/oxide-output/src/bom/mod/export.md) |
