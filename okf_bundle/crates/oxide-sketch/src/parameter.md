---
okf_version: "0.2"
type: Module
title: parameter
description: Parameter table with topological resolution.
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter
language: rust
---

# parameter

Parameter table with topological resolution.

## Docstring

Parameter table with topological resolution.

A sketch's `parameters` field is a `name → source-string` table.
Values can reference each other (`body_w = "= pad_pitch *
(pin_count - 1) + 2mm"`), so resolution must:

1. Parse each source string into an [`ExprNode`].
2. Walk the AST to discover [`ExprNode::Ref`] dependencies.
3. Topologically sort the parameter graph; reject cycles.
4. Evaluate in topo order, accumulating already-resolved
parameters as `Literal(Quantity)` ASTs in the eval context so
later expressions resolve cleanly.

Cycle detection uses an iterative DFS with `Visiting` / `Visited`
colours (Tarjan-style). No third-party graph or solver source
consulted.

## Relationships

| Type | Target |
|------|--------|
| related | [ParameterTable](/crates/oxide-sketch/src/parameter/ParameterTable.md) |
| related | [new](/crates/oxide-sketch/src/parameter/new.md) |
| related | [insert](/crates/oxide-sketch/src/parameter/insert.md) |
| related | [get_raw](/crates/oxide-sketch/src/parameter/get_raw.md) |
| related | [iter](/crates/oxide-sketch/src/parameter/iter.md) |
| related | [len](/crates/oxide-sketch/src/parameter/len.md) |
| related | [is_empty](/crates/oxide-sketch/src/parameter/is_empty.md) |
| related | [new](/crates/oxide-sketch/src/parameter/new.md) |
| related | [insert](/crates/oxide-sketch/src/parameter/insert.md) |
| related | [get_raw](/crates/oxide-sketch/src/parameter/get_raw.md) |
| related | [iter](/crates/oxide-sketch/src/parameter/iter.md) |
| related | [len](/crates/oxide-sketch/src/parameter/len.md) |
| related | [is_empty](/crates/oxide-sketch/src/parameter/is_empty.md) |
| related | [strip_eq_prefix](/crates/oxide-sketch/src/parameter/strip_eq_prefix.md) |
| related | [resolve](/crates/oxide-sketch/src/parameter/resolve.md) |
| related | [Color](/crates/oxide-sketch/src/parameter/Color.md) |
| related | [topo_sort](/crates/oxide-sketch/src/parameter/topo_sort.md) |
| related | [name_borrow](/crates/oxide-sketch/src/parameter/name_borrow.md) |
| related | [collect_deps](/crates/oxide-sketch/src/parameter/collect_deps.md) |
| related | [gather_refs](/crates/oxide-sketch/src/parameter/gather_refs.md) |
| related | [rec](/crates/oxide-sketch/src/parameter/rec.md) |
| related | [approx_eq](/crates/oxide-sketch/src/parameter/approx_eq.md) |
| related | [resolve_single_literal](/crates/oxide-sketch/src/parameter/resolve_single_literal.md) |
| related | [resolve_chained_params](/crates/oxide-sketch/src/parameter/resolve_chained_params.md) |
| related | [resolve_handles_altium_style_eq_prefix](/crates/oxide-sketch/src/parameter/resolve_handles_altium_style_eq_prefix.md) |
| related | [resolve_two_cycle_errors](/crates/oxide-sketch/src/parameter/resolve_two_cycle_errors.md) |
| related | [resolve_three_cycle_errors](/crates/oxide-sketch/src/parameter/resolve_three_cycle_errors.md) |
| related | [resolve_self_reference_errors](/crates/oxide-sketch/src/parameter/resolve_self_reference_errors.md) |
| related | [resolve_canonical_units](/crates/oxide-sketch/src/parameter/resolve_canonical_units.md) |
| related | [resolve_unknown_ref_errors](/crates/oxide-sketch/src/parameter/resolve_unknown_ref_errors.md) |
| related | [resolve_diamond_dependency](/crates/oxide-sketch/src/parameter/resolve_diamond_dependency.md) |
| related | [parameter_table_default_is_empty](/crates/oxide-sketch/src/parameter/parameter_table_default_is_empty.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
