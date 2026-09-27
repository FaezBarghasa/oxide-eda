---
okf_version: "0.2"
type: Function
title: track_with_net
description: "Convenience helper to create a simple `IsTrack AND Net = '<name>'` query."
resource: crates/oxide-rules/src/query_dsl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:11:46Z"
concept_id: crates/oxide-rules/src/query_dsl/track_with_net_1
language: rust
---

# track_with_net

Convenience helper to create a simple `IsTrack AND Net = '<name>'` query.

## Signature

```rust
pub fn track_with_net(net: impl Into<String>) -> QueryPredicate
```

## Visibility

- `pub`

## Docstring

Convenience helper to create a simple `IsTrack AND Net = '<name>'` query.

## Source
Lines 128–137 in `crates/oxide-rules/src/query_dsl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [query_dsl](/crates/oxide-rules/src/query_dsl.md) |
