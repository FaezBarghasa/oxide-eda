---
okf_version: "0.2"
type: Function
title: resolve
description: "Resolve every parameter to a canonical-unit `f64` (mm for"
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/resolve
language: rust
---

# resolve

Resolve every parameter to a canonical-unit `f64` (mm for

## Signature

```rust
pub fn resolve(table: &ParameterTable) -> Result<HashMap<String, f64>, ExprError>
```

## Visibility

- `pub`

## Docstring

Resolve every parameter to a canonical-unit `f64` (mm for
`Length`, rad for `Angle`, raw for `Count`).

Returns `ExprError::Cycle(name)` if the dependency graph contains
a cycle through `name`. Other parse / eval errors propagate as
the corresponding [`ExprError`] variant.

## Source
Lines 76–113 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| calls | [strip_eq_prefix](/crates/oxide-sketch/src/parameter/strip_eq_prefix.md) |
| calls | [parse](/crates/oxide-sketch/src/expr/parse/parse.md) |
| calls | [topo_sort](/crates/oxide-sketch/src/parameter/topo_sort.md) |
| calls | [eval](/crates/oxide-sketch/src/expr/eval/eval.md) |
| called_by | [parameter_table_default_is_empty](/crates/oxide-sketch/src/parameter/parameter_table_default_is_empty.md) |
| called_by | [resolve_canonical_units](/crates/oxide-sketch/src/parameter/resolve_canonical_units.md) |
| called_by | [resolve_chained_params](/crates/oxide-sketch/src/parameter/resolve_chained_params.md) |
| called_by | [resolve_diamond_dependency](/crates/oxide-sketch/src/parameter/resolve_diamond_dependency.md) |
| called_by | [resolve_handles_altium_style_eq_prefix](/crates/oxide-sketch/src/parameter/resolve_handles_altium_style_eq_prefix.md) |
| called_by | [resolve_single_literal](/crates/oxide-sketch/src/parameter/resolve_single_literal.md) |
| called_by | [resolve_two_cycle_errors](/crates/oxide-sketch/src/parameter/resolve_two_cycle_errors.md) |
