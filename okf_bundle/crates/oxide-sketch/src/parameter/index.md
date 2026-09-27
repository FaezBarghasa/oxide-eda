# parameter

## Classs

- [Color](Color.md) — [derive(Clone, Copy, PartialEq, Eq)]
- [ParameterTable](ParameterTable.md) — User-defined parameter table — `name → source-string`. Source

## Functions

- [approx_eq](approx_eq.md)
- [collect_deps](collect_deps.md) — Collect every [`ExprNode::Ref`] name reachable from `name`'s AST.
- [gather_refs](gather_refs.md) — Walk an AST and append every distinct `Ref(name)` name to `out`.
- [get_raw](get_raw.md)
- [get_raw](get_raw_1.md)
- [insert](insert.md) — Insert / overwrite a parameter source string.
- [insert](insert_1.md) — Insert / overwrite a parameter source string.
- [is_empty](is_empty.md)
- [is_empty](is_empty_1.md)
- [iter](iter.md)
- [iter](iter_1.md)
- [len](len.md)
- [len](len_1.md)
- [name_borrow](name_borrow.md) — Borrow a `&str` view of `name` whose lifetime matches `asts`'s
- [new](new.md)
- [new](new_1.md)
- [parameter_table_default_is_empty](parameter_table_default_is_empty.md) — [test]
- [rec](rec.md)
- [resolve](resolve.md) — Resolve every parameter to a canonical-unit `f64` (mm for
- [resolve_canonical_units](resolve_canonical_units.md) — [test]
- [resolve_chained_params](resolve_chained_params.md) — [test]
- [resolve_diamond_dependency](resolve_diamond_dependency.md) — [test]
- [resolve_handles_altium_style_eq_prefix](resolve_handles_altium_style_eq_prefix.md) — [test]
- [resolve_self_reference_errors](resolve_self_reference_errors.md) — [test]
- [resolve_single_literal](resolve_single_literal.md) — [test]
- [resolve_three_cycle_errors](resolve_three_cycle_errors.md) — [test]
- [resolve_two_cycle_errors](resolve_two_cycle_errors.md) — [test]
- [resolve_unknown_ref_errors](resolve_unknown_ref_errors.md) — [test]
- [strip_eq_prefix](strip_eq_prefix.md) — Strip the optional Altium-style leading `=` and surrounding
- [topo_sort](topo_sort.md) — Iterative DFS topological sort. Returns parameter names in an
