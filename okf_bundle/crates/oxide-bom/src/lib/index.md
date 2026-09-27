# lib

## Classs

- [BomComponent](BomComponent.md) — Normalized component candidate for BOM processing.
- [BomContext](BomContext.md) — Input context for BOM generation.
- [BomEngineOptions](BomEngineOptions.md) — Engine options for BOM table generation.
- [BomGrouping](BomGrouping.md) — How to group components in the BOM.
- [BomIssueSeverity](BomIssueSeverity.md) — [derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
- [BomMetadata](BomMetadata.md) — Metadata propagated to BOM outputs.
- [BomRow](BomRow.md) — A single row in the generated BOM table.
- [BomRule](BomRule.md) — Supported BOM validation rules.
- [BomRuleOptions](BomRuleOptions.md) — Rule-level on/off switches for BOM validation.
- [BomTable](BomTable.md) — Generated BOM table.
- [BomValidationIssue](BomValidationIssue.md) — Single validation finding produced by BOM rules.
- [BomValidationReport](BomValidationReport.md) — Validation report emitted after BOM table generation.

## Functions

- [build_table](build_table.md) — Build a BOM table from a normalized BOM context.
- [can_include_excluded_from_bom_and_not_on_board](can_include_excluded_from_bom_and_not_on_board.md) — [test]
- [component](component.md)
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [error_count](error_count.md)
- [error_count](error_count_1.md)
- [filters_dnp_by_default](filters_dnp_by_default.md) — [test]
- [first_reference](first_reference.md)
- [groups_by_value_and_footprint](groups_by_value_and_footprint.md) — [test]
- [has_errors](has_errors.md)
- [has_errors](has_errors_1.md)
- [has_populated_field](has_populated_field.md)
- [is_enabled](is_enabled.md)
- [is_enabled](is_enabled_1.md)
- [is_fitted](is_fitted.md)
- [is_fitted](is_fitted_1.md)
- [sorts_references_naturally_when_grouped](sorts_references_naturally_when_grouped.md) — [test]
- [sorts_references_without_or_with_oversized_numeric_tails](sorts_references_without_or_with_oversized_numeric_tails.md) — [test]
- [sorts_rows_naturally_when_flat](sorts_rows_naturally_when_flat.md) — [test]
- [sorts_rows_naturally_when_grouped_across_rows](sorts_rows_naturally_when_grouped_across_rows.md) — [test]
- [validate_table](validate_table.md) — Run quality rules against BOM input/output and return a validation report.
- [validation_can_disable_missing_footprint_rule](validation_can_disable_missing_footprint_rule.md) — [test]
- [validation_reports_duplicate_designators](validation_reports_duplicate_designators.md) — [test]
- [warning_count](warning_count.md)
- [warning_count](warning_count_1.md)
