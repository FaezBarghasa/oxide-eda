# mod

## Classs

- [BomColumn](BomColumn.md) — Column identifiers for the BOM.
- [BomError](BomError.md) — [derive(Debug, Error)]
- [BomExporter](BomExporter.md) — [derive(Default)]
- [BomFormat](BomFormat.md) — Output format for the BOM.
- [BomOptions](BomOptions.md) — Configuration for BOM export.
- [BomOutput](BomOutput.md) — The output of a BOM export.
- [VariantFieldKind](VariantFieldKind.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]

## Functions

- [base_custom_properties_prefers_fitted_over_dnp_regardless_of_order](base_custom_properties_prefers_fitted_over_dnp_regardless_of_order.md) — `resolve_base_variant_fitted`'s `custom_properties` loop used to return
- [bom_format_resolves_from_output_path](bom_format_resolves_from_output_path.md) — [test]
- [build_bom_context](build_bom_context.md)
- [build_validation_context](build_validation_context.md)
- [component_is_exported](component_is_exported.md)
- [contradictory_variant_fields_resolve_deterministically](contradictory_variant_fields_resolve_deterministically.md) — `symbol.fields` is a `HashMap`, so a symbol carrying contradictory
- [csv_emits_rfc4180](csv_emits_rfc4180.md) — [test]
- [default](default.md)
- [default](default_1.md)
- [engine_options_from_opts](engine_options_from_opts.md)
- [export](export.md)
- [export](export_1.md)
- [field_value_ci](field_value_ci.md) — Case-insensitive field lookup, deterministic on a case collision.
- [field_value_ci_falls_back_to_smallest_key_without_exact_match](field_value_ci_falls_back_to_smallest_key_without_exact_match.md) — With no exact-case match, the fallback must still be a pure function
- [field_value_ci_prefers_exact_case_on_collision](field_value_ci_prefers_exact_case_on_collision.md) — A symbol carrying both `Fitted` and `fitted` used to resolve by
- [from](from.md)
- [from](from_1.md)
- [from_output_path](from_output_path.md) — Resolve BOM format from output file extension.
- [from_output_path](from_output_path_1.md) — Resolve BOM format from output file extension.
- [header](header.md)
- [header](header_1.md)
- [html_self_contained](html_self_contained.md) — [test]
- [parse_bool_field](parse_bool_field.md)
- [parse_variant_field_key](parse_variant_field_key.md)
- [property_variant_kind](property_variant_kind.md)
- [rank_fitted_over_dnp](rank_fitted_over_dnp.md) — Rank a parsed fit-state field so Fitted always outranks DNP on a tie.
- [resolve_base_variant_fitted](resolve_base_variant_fitted.md)
- [resolve_variant_fitted](resolve_variant_fitted.md)
- [resolve_variant_fitted_from_fields](resolve_variant_fitted_from_fields.md) — Fit state for `active_variant` taken from `symbol.fields`.
- [resolve_variant_fitted_from_property_overrides](resolve_variant_fitted_from_property_overrides.md)
- [rollup](rollup.md) — Walks every sheet in the ExportContext, aggregates components according
- [test_symbol](test_symbol.md)
- [variant_fitted_falls_back_to_base_when_variant_value_unparseable](variant_fitted_falls_back_to_base_when_variant_value_unparseable.md) — [test]
- [variant_fitted_prefers_property_override](variant_fitted_prefers_property_override.md) — [test]
- [xlsx_produces_nonempty_bytes](xlsx_produces_nonempty_bytes.md) — [test]
