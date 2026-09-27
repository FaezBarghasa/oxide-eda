---
okf_version: "0.2"
type: Module
title: query_dsl
description: Query Filter DSL parser and AST evaluator.
resource: crates/oxide-rules/src/query_dsl.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:11:46Z"
concept_id: crates/oxide-rules/src/query_dsl
language: rust
---

# query_dsl

Query Filter DSL parser and AST evaluator.

## Docstring

Query Filter DSL parser and AST evaluator.

Enables instantaneous filtering and selection of dense PCB primitives using
natural engineering filter queries, e.g.:
`IsTrack AND Net = 'DDR4*' AND Layer = 'Top' AND Width >= 0.2`

## Relationships

| Type | Target |
|------|--------|
| related | [QueryTargetType](/crates/oxide-rules/src/query_dsl/QueryTargetType.md) |
| related | [ComparisonOp](/crates/oxide-rules/src/query_dsl/ComparisonOp.md) |
| related | [QueryPredicate](/crates/oxide-rules/src/query_dsl/QueryPredicate.md) |
| related | [PrimitiveEvaluationContext](/crates/oxide-rules/src/query_dsl/PrimitiveEvaluationContext.md) |
| related | [evaluate](/crates/oxide-rules/src/query_dsl/evaluate.md) |
| related | [evaluate](/crates/oxide-rules/src/query_dsl/evaluate.md) |
| related | [QueryParser](/crates/oxide-rules/src/query_dsl/QueryParser.md) |
| related | [track_with_net](/crates/oxide-rules/src/query_dsl/track_with_net.md) |
| related | [net_like](/crates/oxide-rules/src/query_dsl/net_like.md) |
| related | [track_with_net](/crates/oxide-rules/src/query_dsl/track_with_net.md) |
| related | [net_like](/crates/oxide-rules/src/query_dsl/net_like.md) |
| related | [test_query_predicate_evaluation](/crates/oxide-rules/src/query_dsl/test_query_predicate_evaluation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
