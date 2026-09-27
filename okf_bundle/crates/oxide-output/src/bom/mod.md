---
okf_version: "0.2"
type: Module
title: bom
description: "BOM (Bill of Materials) export — CSV, HTML, XLSX formats."
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod
language: rust
---

# bom

BOM (Bill of Materials) export — CSV, HTML, XLSX formats.

## Docstring

BOM (Bill of Materials) export — CSV, HTML, XLSX formats.

See `OUTPUT_PLAN.md` §8. Walks every sheet in an ExportContext, aggregates
components, skips power ports and DNP symbols, and emits in one of three formats.

## Relationships

| Type | Target |
|------|--------|
| related | [BomExporter](/crates/oxide-output/src/bom/mod/BomExporter.md) |
| related | [BomOptions](/crates/oxide-output/src/bom/mod/BomOptions.md) |
| related | [default](/crates/oxide-output/src/bom/mod/default.md) |
| related | [default](/crates/oxide-output/src/bom/mod/default.md) |
| related | [VariantFieldKind](/crates/oxide-output/src/bom/mod/VariantFieldKind.md) |
| related | [BomColumn](/crates/oxide-output/src/bom/mod/BomColumn.md) |
| related | [header](/crates/oxide-output/src/bom/mod/header.md) |
| related | [header](/crates/oxide-output/src/bom/mod/header.md) |
| related | [BomFormat](/crates/oxide-output/src/bom/mod/BomFormat.md) |
| related | [from_output_path](/crates/oxide-output/src/bom/mod/from_output_path.md) |
| related | [from_output_path](/crates/oxide-output/src/bom/mod/from_output_path.md) |
| related | [BomOutput](/crates/oxide-output/src/bom/mod/BomOutput.md) |
| related | [BomError](/crates/oxide-output/src/bom/mod/BomError.md) |
| related | [from](/crates/oxide-output/src/bom/mod/from.md) |
| related | [from](/crates/oxide-output/src/bom/mod/from.md) |
| related | [rollup](/crates/oxide-output/src/bom/mod/rollup.md) |
| related | [engine_options_from_opts](/crates/oxide-output/src/bom/mod/engine_options_from_opts.md) |
| related | [build_bom_context](/crates/oxide-output/src/bom/mod/build_bom_context.md) |
| related | [component_is_exported](/crates/oxide-output/src/bom/mod/component_is_exported.md) |
| related | [build_validation_context](/crates/oxide-output/src/bom/mod/build_validation_context.md) |
| related | [parse_bool_field](/crates/oxide-output/src/bom/mod/parse_bool_field.md) |
| related | [parse_variant_field_key](/crates/oxide-output/src/bom/mod/parse_variant_field_key.md) |
| related | [property_variant_kind](/crates/oxide-output/src/bom/mod/property_variant_kind.md) |
| related | [rank_fitted_over_dnp](/crates/oxide-output/src/bom/mod/rank_fitted_over_dnp.md) |
| related | [field_value_ci](/crates/oxide-output/src/bom/mod/field_value_ci.md) |
| related | [resolve_variant_fitted_from_property_overrides](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_property_overrides.md) |
| related | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
| related | [resolve_variant_fitted_from_fields](/crates/oxide-output/src/bom/mod/resolve_variant_fitted_from_fields.md) |
| related | [resolve_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_variant_fitted.md) |
| related | [export](/crates/oxide-output/src/bom/mod/export.md) |
| related | [export](/crates/oxide-output/src/bom/mod/export.md) |
| related | [test_symbol](/crates/oxide-output/src/bom/mod/test_symbol.md) |
| related | [variant_fitted_prefers_property_override](/crates/oxide-output/src/bom/mod/variant_fitted_prefers_property_override.md) |
| related | [variant_fitted_falls_back_to_base_when_variant_value_unparseable](/crates/oxide-output/src/bom/mod/variant_fitted_falls_back_to_base_when_variant_value_unparseable.md) |
| related | [contradictory_variant_fields_resolve_deterministically](/crates/oxide-output/src/bom/mod/contradictory_variant_fields_resolve_deterministically.md) |
| related | [field_value_ci_prefers_exact_case_on_collision](/crates/oxide-output/src/bom/mod/field_value_ci_prefers_exact_case_on_collision.md) |
| related | [field_value_ci_falls_back_to_smallest_key_without_exact_match](/crates/oxide-output/src/bom/mod/field_value_ci_falls_back_to_smallest_key_without_exact_match.md) |
| related | [base_custom_properties_prefers_fitted_over_dnp_regardless_of_order](/crates/oxide-output/src/bom/mod/base_custom_properties_prefers_fitted_over_dnp_regardless_of_order.md) |
| related | [csv_emits_rfc4180](/crates/oxide-output/src/bom/mod/csv_emits_rfc4180.md) |
| related | [html_self_contained](/crates/oxide-output/src/bom/mod/html_self_contained.md) |
| related | [xlsx_produces_nonempty_bytes](/crates/oxide-output/src/bom/mod/xlsx_produces_nonempty_bytes.md) |
| related | [bom_format_resolves_from_output_path](/crates/oxide-output/src/bom/mod/bom_format_resolves_from_output_path.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
