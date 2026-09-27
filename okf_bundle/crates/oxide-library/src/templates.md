---
okf_version: "0.2"
type: Module
title: templates
description: "Parameter templates — class-typed schemas that constrain a component's"
resource: crates/oxide-library/src/templates.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/templates
language: rust
---

# templates

Parameter templates — class-typed schemas that constrain a component's

## Docstring

Parameter templates — class-typed schemas that constrain a component's
`parameters` map.

Per `v0.9-refactor-2-plan.md` §4, every `Component` has a class
(e.g. "resistor", "opamp"); the matching template lists which parameters
are required vs optional and what kind of value (text, number, measurement)
each carries.

Resolution order (§4.3):
1. per-library override (inside `*.snxlib/templates/<class>.toml`),
2. global override (`<config_dir>/oxide/templates/<class>.toml`) — *not
loaded by this crate; `TemplateRegistry::load_global_dir` is called by
the app shell during start-up*,
3. the bundled built-in.

`TemplateRegistry::new_with_builtins` ships with five starter classes:
resistor, capacitor, inductor, opamp, generic. Anything else falls through
to "no template" (the `validate` step then trivially passes).

## Relationships

| Type | Target |
|------|--------|
| related | [ParamKind](/crates/oxide-library/src/templates/ParamKind.md) |
| related | [ParamSlot](/crates/oxide-library/src/templates/ParamSlot.md) |
| related | [ParameterTemplate](/crates/oxide-library/src/templates/ParameterTemplate.md) |
| related | [parse](/crates/oxide-library/src/templates/parse.md) |
| related | [parse](/crates/oxide-library/src/templates/parse.md) |
| related | [TemplateViolation](/crates/oxide-library/src/templates/TemplateViolation.md) |
| related | [fmt](/crates/oxide-library/src/templates/fmt.md) |
| related | [fmt](/crates/oxide-library/src/templates/fmt.md) |
| related | [TemplateRegistry](/crates/oxide-library/src/templates/TemplateRegistry.md) |
| related | [new](/crates/oxide-library/src/templates/new.md) |
| related | [new_with_builtins](/crates/oxide-library/src/templates/new_with_builtins.md) |
| related | [insert_global](/crates/oxide-library/src/templates/insert_global.md) |
| related | [insert_for_library](/crates/oxide-library/src/templates/insert_for_library.md) |
| related | [resolve](/crates/oxide-library/src/templates/resolve.md) |
| related | [validate_params](/crates/oxide-library/src/templates/validate_params.md) |
| related | [new](/crates/oxide-library/src/templates/new.md) |
| related | [new_with_builtins](/crates/oxide-library/src/templates/new_with_builtins.md) |
| related | [insert_global](/crates/oxide-library/src/templates/insert_global.md) |
| related | [insert_for_library](/crates/oxide-library/src/templates/insert_for_library.md) |
| related | [resolve](/crates/oxide-library/src/templates/resolve.md) |
| related | [validate_params](/crates/oxide-library/src/templates/validate_params.md) |
| related | [kind_of](/crates/oxide-library/src/templates/kind_of.md) |
| related | [check_slot](/crates/oxide-library/src/templates/check_slot.md) |
| related | [builtin_registry_resolves_resistor](/crates/oxide-library/src/templates/builtin_registry_resolves_resistor.md) |
| related | [builtin_registry_resolves_capacitor_inductor_opamp_generic](/crates/oxide-library/src/templates/builtin_registry_resolves_capacitor_inductor_opamp_generic.md) |
| related | [unknown_class_resolves_to_none](/crates/oxide-library/src/templates/unknown_class_resolves_to_none.md) |
| related | [per_lib_override_takes_precedence](/crates/oxide-library/src/templates/per_lib_override_takes_precedence.md) |
| related | [validate_passes_when_no_template_registered](/crates/oxide-library/src/templates/validate_passes_when_no_template_registered.md) |
| related | [validate_flags_missing_required](/crates/oxide-library/src/templates/validate_flags_missing_required.md) |
| related | [validate_flags_wrong_kind](/crates/oxide-library/src/templates/validate_flags_wrong_kind.md) |
| related | [validate_flags_wrong_unit](/crates/oxide-library/src/templates/validate_flags_wrong_unit.md) |
| related | [template_round_trip_through_toml](/crates/oxide-library/src/templates/template_round_trip_through_toml.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
