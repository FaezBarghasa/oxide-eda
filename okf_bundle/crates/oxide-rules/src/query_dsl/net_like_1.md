---
okf_version: "0.2"
type: Function
title: net_like
description: "Convenience helper to create a wildcard `Net LIKE '<pattern>'` query."
resource: crates/oxide-rules/src/query_dsl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:11:46Z"
concept_id: crates/oxide-rules/src/query_dsl/net_like_1
language: rust
---

# net_like

Convenience helper to create a wildcard `Net LIKE '<pattern>'` query.

## Signature

```rust
pub fn net_like(pattern: impl Into<String>) -> QueryPredicate
```

## Visibility

- `pub`

## Docstring

Convenience helper to create a wildcard `Net LIKE '<pattern>'` query.

## Source
Lines 140–146 in `crates/oxide-rules/src/query_dsl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [query_dsl](/crates/oxide-rules/src/query_dsl.md) |
