# query_dsl

## Classs

- [ComparisonOp](ComparisonOp.md) — Comparison operator for numeric and string fields.
- [PrimitiveEvaluationContext](PrimitiveEvaluationContext.md) — Query context describing an arbitrary PCB primitive being evaluated.
- [QueryParser](QueryParser.md) — Helper parser for building simple AST expressions from tokens.
- [QueryPredicate](QueryPredicate.md) — Single query filter predicate.
- [QueryTargetType](QueryTargetType.md) — Target object primitive type in query predicate.

## Functions

- [evaluate](evaluate.md) — Evaluate this predicate against a primitive context.
- [evaluate](evaluate_1.md) — Evaluate this predicate against a primitive context.
- [net_like](net_like.md) — Convenience helper to create a wildcard `Net LIKE '<pattern>'` query.
- [net_like](net_like_1.md) — Convenience helper to create a wildcard `Net LIKE '<pattern>'` query.
- [test_query_predicate_evaluation](test_query_predicate_evaluation.md) — [test]
- [track_with_net](track_with_net.md) — Convenience helper to create a simple `IsTrack AND Net = '<name>'` query.
- [track_with_net](track_with_net_1.md) — Convenience helper to create a simple `IsTrack AND Net = '<name>'` query.
