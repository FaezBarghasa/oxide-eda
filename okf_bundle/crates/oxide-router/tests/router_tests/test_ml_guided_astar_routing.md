---
okf_version: "0.2"
type: Function
title: test_ml_guided_astar_routing
description: "[test]"
resource: crates/oxide-router/tests/router_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:07:42Z"
concept_id: crates/oxide-router/tests/router_tests/test_ml_guided_astar_routing
language: rust
---

# test_ml_guided_astar_routing

[test]

## Signature

```rust
fn test_ml_guided_astar_routing()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 201–222 in `crates/oxide-router/tests/router_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router_tests](/crates/oxide-router/tests/router_tests.md) |
| calls | [mock_board](/crates/oxide-router/tests/router_tests/mock_board.md) |
| calls | [find_astar_path_with_ml](/crates/oxide-router/src/interactive/astar/find_astar_path_with_ml.md) |
