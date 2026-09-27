# templates

## Classs

- [ParameterTemplate](ParameterTemplate.md) — Class-typed parameter schema.
- [ParamKind](ParamKind.md) — Kind of value a parameter slot accepts.
- [ParamSlot](ParamSlot.md) — One parameter slot in a [`ParameterTemplate`].
- [TemplateRegistry](TemplateRegistry.md) — Per-library + global template registry.
- [TemplateViolation](TemplateViolation.md) — Why a parameter map fails to validate against its template.

## Functions

- [builtin_registry_resolves_capacitor_inductor_opamp_generic](builtin_registry_resolves_capacitor_inductor_opamp_generic.md) — [test]
- [builtin_registry_resolves_resistor](builtin_registry_resolves_resistor.md) — [test]
- [check_slot](check_slot.md)
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [insert_for_library](insert_for_library.md) — Add (or replace) a per-library override.
- [insert_for_library](insert_for_library_1.md) — Add (or replace) a per-library override.
- [insert_global](insert_global.md) — Add (or replace) a global override for `class`.
- [insert_global](insert_global_1.md) — Add (or replace) a global override for `class`.
- [kind_of](kind_of.md)
- [new](new.md) — Empty registry — no templates resolved. Mostly useful for tests.
- [new](new_1.md) — Empty registry — no templates resolved. Mostly useful for tests.
- [new_with_builtins](new_with_builtins.md) — Registry seeded with the five bundled built-ins.
- [new_with_builtins](new_with_builtins_1.md) — Registry seeded with the five bundled built-ins.
- [parse](parse.md)
- [parse](parse_1.md)
- [per_lib_override_takes_precedence](per_lib_override_takes_precedence.md) — [test]
- [resolve](resolve.md) — Lookup order (per plan §4.3):
- [resolve](resolve_1.md) — Lookup order (per plan §4.3):
- [template_round_trip_through_toml](template_round_trip_through_toml.md) — [test]
- [unknown_class_resolves_to_none](unknown_class_resolves_to_none.md) — [test]
- [validate_flags_missing_required](validate_flags_missing_required.md) — [test]
- [validate_flags_wrong_kind](validate_flags_wrong_kind.md) — [test]
- [validate_flags_wrong_unit](validate_flags_wrong_unit.md) — [test]
- [validate_params](validate_params.md) — Validate a parameter map against its class template (looked up via
- [validate_params](validate_params_1.md) — Validate a parameter map against its class template (looked up via
- [validate_passes_when_no_template_registered](validate_passes_when_no_template_registered.md) — [test]
