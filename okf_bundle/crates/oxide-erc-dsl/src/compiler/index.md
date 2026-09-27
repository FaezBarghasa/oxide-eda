# compiler

## Classs

- [CompiledExpr](CompiledExpr.md) — Expression after compilation/preprocessing.
- [CompiledHelper](CompiledHelper.md) — A compiled helper call. For regex-capable helpers, regexes are compiled once.
- [CompiledRule](CompiledRule.md) — A fully compiled DSL rule.
- [Subject](Subject.md)
- [Value](Value.md)

## Functions

- [compile](compile.md) — Compile all rules. Continues compiling independent rules and returns all
- [compile_expr](compile_expr.md)
- [compile_rule](compile_rule.md)
- [eval_expr](eval_expr.md)
- [eval_field_cmp](eval_field_cmp.md)
- [eval_field_matches](eval_field_matches.md)
- [eval_fn](eval_fn.md) — Clones the evaluator closure for use with `engine::run_all_with_dsl`.
- [eval_fn](eval_fn_1.md) — Clones the evaluator closure for use with `engine::run_all_with_dsl`.
- [eval_helper](eval_helper.md)
- [evaluate_rule](evaluate_rule.md)
- [fallback_kind](fallback_kind.md)
- [is_driving_pin](is_driving_pin.md)
- [map_applicability](map_applicability.md)
- [map_scope](map_scope.md)
- [map_severity](map_severity.md)
- [map_target](map_target.md)
- [normalize](normalize.md)
- [parse_pin_type](parse_pin_type.md)
- [pin_type_name](pin_type_name.md)
- [resolve_field](resolve_field.md)
- [subject_ref](subject_ref.md)
- [to_eval_fns](to_eval_fns.md) — Convert compiled rules into engine evaluator closures.
